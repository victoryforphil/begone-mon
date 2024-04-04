use std::fs::File;
use begone::{BegoneConfig, Killer};
use clap::Parser;
use log::error;
use std::path::PathBuf;
use simplelog::*;
use tracing::info;

/// Binary to create a run a simple begone scan on a given single directory
/// Args
/// - root path: The path to scan
/// - kill pattern: The patterns to search for and delete (default = node_modules)
/// - dry run: If true, don't delete anything (default = true)
/// - verbose: If true, print out all files being scanned (default = true)
/// - threads: The number of threads to use (default = 1)
/// - batch size: The number of files to store in memory before processing (default = 100)
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct BegonArgs {
    #[arg(short, long)]
    root_path: PathBuf
}

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("begone.log").unwrap(),
        ),
    ])
    .unwrap();
    info!("Starting Begone Client");
    let mut config = BegoneConfig::node_module_default();
    // Override the default root path with the one provided
    let args = BegonArgs::parse();
    config.root_paths = vec![args.root_path.to_str().unwrap().to_string()];
    info!(" Config: {:#?}", config);

    let discovery = begone::Discovery::search(&config);

    // Print how many results found 
    info!("Found {} results", discovery.results.len());

    let kill_results = Killer::kill(discovery.results);
    // Print any non-Killed results
    for result in kill_results.iter(){
        if let begone::KillResult::NotKilled(reason) = &result.result{
            error!("Failed to kill: \n\t-{:?} \n\t-because: {:?}", result.request, reason);
        }
    }
    
}
