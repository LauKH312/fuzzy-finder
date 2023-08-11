use std::{
    collections::BTreeSet,
    error::Error,
    path::{Path, PathBuf},
};

use eframe::egui;

use crate::fileops;

pub struct App {
    search: String,
    paths: BTreeSet<PathBuf>,
    cached_paths: Option<BTreeSet<PathBuf>>,
}

impl App {
    pub fn new(dir: PathBuf) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            search: String::new(),
            paths: fileops::get_all_in_dir_parallel(&dir)?,
            cached_paths: None,
        })
    }

    fn cache_paths(&mut self) {
        let search = self.paths.iter().filter(|path| {
            path.to_str()
                .unwrap()
                .to_lowercase()
                .contains(&self.search.to_lowercase())
        });

        self.cached_paths = Some(BTreeSet::from_iter(search.clone().cloned()));
    }

    fn print_searched_paths(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let max_items = 25;

        if self.search.len() % 5 == 0 && !self.search.is_empty() {
            self.cache_paths();
        }

        let shown_paths = match &self.cached_paths {
            Some(paths) => paths.iter(),
            None => self.paths.iter(),
        }
        .filter(|path| {
            path.to_str()
                .map(|s| s.to_lowercase().contains(&self.search.to_lowercase()))
                .unwrap_or(false)
        })
        .take(max_items);

        for path in shown_paths {
            if ui.button(path.to_str().unwrap()).clicked() {
                spawn_process(path, frame);
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
        setup_fonts(ctx);

        ctx.set_pixels_per_point(1.5);

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.text_edit_singleline(&mut self.search).request_focus();
                });

                ui.vertical(|ui| {
                    self.print_searched_paths(ui, frame);

                    // parse user commands
                    if self.search.ends_with('!') {
                        frame.close();
                        std::process::exit(0);
                    }
                });
            });
        });
    }
}

fn spawn_process(path: &Path, frame: &mut eframe::Frame) -> ! {
    std::process::Command::new("explorer")
        .arg(path.to_str().unwrap().replace('/', r"\"))
        .spawn()
        .expect("Error opening file explorer");

    frame.close();
    std::process::exit(0);
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Inconsolata".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "C:/Users/Laurits/Documents/Hobby/Kode/Rust/fuzzie/assets/fonts/Inconsolata.ttf"
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Inconsolata".to_owned());

    // set font size

    ctx.set_fonts(fonts);
}
