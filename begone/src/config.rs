use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BegoneConfig{
    pub root_paths: Vec<String>,
    pub kill_patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub dry_run: bool,
    pub verbose: bool,
    pub threads: usize,
    pub batch_size: usize,
}

impl BegoneConfig {
    pub fn new() -> Self {
        BegoneConfig {
            root_paths: vec![],
            kill_patterns: vec![],
            ignore_patterns: vec![],
            dry_run: true,
            verbose: true,
            threads: 1,
            batch_size: 100,
        }
    }

    pub fn node_module_default() -> Self {
        let cwd = std::env::current_dir().unwrap();
        BegoneConfig {
            root_paths: vec![cwd.to_str().unwrap().to_string()],
            kill_patterns: vec!["node_modules".to_string()],
            ignore_patterns: vec![],
            dry_run: true,
            verbose: true,
            threads: 1,
            batch_size: 100,
        }
    }
}

impl Default for BegoneConfig {
    fn default() -> Self {
        Self::new()
    }
}