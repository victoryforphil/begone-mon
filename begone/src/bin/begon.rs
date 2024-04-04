use std::fs::File;

use simplelog::*;
use tracing::info;



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
    info!("Starting Wake Client");

    
}
