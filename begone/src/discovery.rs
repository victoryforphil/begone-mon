use std::path::Path;

use crate::{config, BegoneConfig};
use tracing::debug;
use tracing_subscriber::field::debug;
use walkdir::WalkDir;
#[derive(Debug)]
pub struct DiscoveyResult {
    pub url: String
}

impl DiscoveyResult {
    pub fn new(url: String) -> DiscoveyResult {
        DiscoveyResult { url }
    }

    pub fn from_path(path: &Path) -> DiscoveyResult {
        DiscoveyResult {
            url: path.to_str().unwrap().to_string(),
        }
    }

   
    
}
#[derive(Debug)]
pub struct Discovery {
    pub results: Vec<DiscoveyResult>,
}

impl Discovery{
    pub fn search(config: &BegoneConfig) -> Discovery {
        let mut results = vec![];
        for root in config.root_paths.iter() {
            for entry in WalkDir::new(root) {
               
                let entry = entry.unwrap();
                let file_name = entry.file_name().to_str().unwrap();
                let mut kill = false;

                for pattern in config.kill_patterns.iter() {
                    if file_name.contains(pattern) {
                        kill = true;
                        debug!("Found a kill pattern: {:?} in {:?}", pattern, entry.path());
                        break;
                    }
                }

               if kill {
                    results.push(DiscoveyResult::from_path(entry.path()));
                }
            }
        }
        Discovery { results }
    }
}