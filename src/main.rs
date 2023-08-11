use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use clap::arg;
use clap::command;
use clap::Parser;
use eframe::NativeOptions;

mod app;
mod fileops;

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    directory: Option<String>,

    #[arg(short, long, default_value = "false")]
    cache: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let dir: PathBuf = match args.directory {
        Some(dir) => Path::new(&dir).to_path_buf(),
        None => std::env::current_dir()?,
    };

    let options = NativeOptions {
        always_on_top: true,
        transparent: true,
        initial_window_size: Some(egui::Vec2::new(900.0, 700.0)),
        ..Default::default()
    };

    if args.cache {
        let tree = fileops::get_all_in_dir_parallel(&dir)?;
        let arr = tree.iter().collect::<Vec<&PathBuf>>();
        let serialized = bincode::serialize(&arr)?;
        fs::write(fileops::CACHE_DIR, serialized)?;
    }

    eframe::run_native(
        "Fuzzie",
        options,
        // allocate space for app (probably)
        Box::new(|_cc| Box::new(app::App::new(dir).unwrap())),
    )?;

    Ok(())
}
