use egui::epaint::ahash::HashSet;
use rayon::prelude::*;

use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

pub const CACHE_DIR: &str = "C:/ProgramData/fuzzie-finder/cache";

pub fn get_all_in_dir_parallel(dir: &Path) -> Result<BTreeSet<PathBuf>, Box<dyn Error>> {
    let paths = Mutex::new(BTreeSet::new());

    let format_whitelist = vec![
        // SOURCE CODE FORMATS
        "ada",
        "asm",
        "c",
        "cpp",
        "c++",
        "cxx",
        "cs",
        "css",
        "dart",
        "fs",
        "go",
        "h",
        "hpp",
        "h++",
        "hxx",
        "html",
        "java",
        "js",
        "jsx",
        "kt",
        "lua",
        "php",
        "py",
        "rb",
        "rs",
        "s",
        "sass",
        "scss",
        "scala",
        "sh",
        "svelte",
        "swift",
        "ts",
        "tsx",
        "zig",
        // DATA FORMATS
        "bat",
        "csv",
        "env",
        "json",
        "lock",
        "xml",
        // CONFIG FORMATS
        "cfg",
        "conf",
        "editorconfig",
        "git",
        "gitattributes",
        "gitconfig",
        "gitignore",
        "gitkeep",
        "gitmodules",
        "ignore",
        "ini",
        "toml",
        "yaml",
        "yml",
        // OTHER TEXT FORMATS
        "log",
        "md",
        "txt",
    ];

    if Path::new(CACHE_DIR).exists() {
        let serialized = fs::read(CACHE_DIR)?;
        let list: Vec<PathBuf> = bincode::deserialize(&serialized)?;
        let tree = list.into_iter().collect();
        return Ok(tree);
    };

    let allowed_extensions = HashSet::from_iter(format_whitelist.into_iter());

    walkdir::WalkDir::new(dir.canonicalize().expect("Error canonicalizing path"))
        .into_iter()
        .par_bridge()
        .filter(|entry| {
            // Make sure entry isn't a binary file or other coded formats
            let entry = entry.as_ref().unwrap();
            let path = entry.path();
            let ext = path.extension();
            match ext {
                Some(ext) => match ext.to_str() {
                    Some(ext) => allowed_extensions.contains(&ext),
                    _ => false,
                },
                None => true,
            }
        })
        .for_each(|entry| {
            // Process entry
            let mut lock = paths.lock().unwrap();
            lock.insert(entry.unwrap().into_path());
        });

    Ok(paths.into_inner()?)
}
