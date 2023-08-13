use egui::epaint::ahash::HashSet;
use rayon::prelude::*;

use std::{
    collections::BTreeSet,
    error::Error,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

pub const CACHE_DIR: &str = "C:/ProgramData/fuzzyfinder/cache";

pub fn get_all_in_dir_parallel(dir: &Path) -> Result<BTreeSet<PathBuf>, Box<dyn Error>> {
    let paths = Mutex::new(BTreeSet::new());

    let format_whitelist = include_str!("../assets/data/whitelist.txt")
        .split('\n')
        .filter(|line| !line.starts_with('#')) // allow comments
        .collect::<Vec<&str>>();

    // let folder_blacklist = include_str!("../assets/data/folder_blacklist.txt")
    //     .split('\n')
    //     .filter(|line| !line.starts_with('#')) // allow comments
    //     .collect::<Vec<&str>>();

    if Path::new(CACHE_DIR).exists() {
        let serialized = fs::read(CACHE_DIR)?;
        let list: Vec<PathBuf> = bincode::deserialize(&serialized)?;
        let tree = list.into_iter().collect();
        return Ok(tree);
    };

    let allowed_extensions = HashSet::from_iter(format_whitelist.into_iter());

    walkdir::WalkDir::new(dir)
        .into_iter()
        .par_bridge()
        .filter(|entry| {
            // Make sure entry isn't a binary file or other coded formats
            let entry = entry.as_ref().unwrap();
            let path = entry.path();

            match path.extension() {
                Some(ext) => match ext.to_str() {
                    Some(ext) => allowed_extensions.contains(&ext),
                    _ => false,
                },
                None => true, // todo: filter out blacklisted folders
            }
        })
        .for_each(|entry| {
            // Process entry
            let mut lock = paths.lock().unwrap();
            lock.insert(entry.unwrap().into_path());
        });

    Ok(paths.into_inner()?)
}
