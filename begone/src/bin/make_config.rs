use std::fs::File;


use begone::BegoneConfig;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use tracing::*;
use inquire::*;

fn get_roots(config: &mut BegoneConfig){
    let mut root_paths = vec![];
    info!("Getting Root Paths");
    //TODO: Default to CWD [VFP]
    //TODO: Auto-complete paths [VFP]
    //TODO: Validate paths exists [VFP]
    let first_path = Text::new("Enter a root path").prompt();

    root_paths.push(first_path.unwrap());

    let mut path = Text::new("Enter another root path").prompt();
    let mut path = path.unwrap_or_default();
    while path.clone().len() > 0{
        info!("Got a path: {:?}", path);
        root_paths.push(path.clone());
        path = Text::new("Enter another root path").prompt().unwrap_or_default();
    }
    info!("Got an empty path: Root Paths: {:?}", root_paths);

    let ans = Confirm::new("Roots Look Correct?")
    .with_default(false)
    .with_help_message(root_paths.join("\n").as_str())
    .prompt();

    if ans.unwrap(){
        info!("Roots Look Good, Setting Config");
        config.root_paths = root_paths;
    } else {
        info!("Roots Look Bad");
        get_roots(config);
    }
}

fn get_kill_patterns(config: &mut BegoneConfig){
    let mut kill_patterns = vec![];
    info!("Getting Kill Patterns");

    let first_pattern = Text::new("Enter a kill pattern").with_default("node_modules").prompt();
    kill_patterns.push(first_pattern.unwrap());

    let mut pattern = Text::new("Enter another kill pattern (empty to stop").prompt();
    let mut pattern = pattern.unwrap_or_default();
    while pattern.clone().len() > 0{
        info!("Got a pattern: {:?}", pattern);
        kill_patterns.push(pattern.clone());
        pattern = Text::new("Enter another kill pattern (empty to stop").prompt().unwrap_or_default();
    }

    info!("Got an empty pattern: Kill Patterns: {:?}", kill_patterns);

    let ans = Confirm::new("Kill Patterns Look Correct?")
    .with_default(false)
    .with_help_message(kill_patterns.join("\n").as_str())
    .prompt();

    if ans.unwrap(){
        info!("Kill Patterns Look Good, Setting Config");
        config.kill_patterns = kill_patterns;
    } else {
        info!("Kill Patterns Look Bad");
        get_kill_patterns(config);
    }

}




fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("make_config.log").unwrap(),
        ),
    ])
    .unwrap();
    info!("Staring Make Config Interactive CLI");
    
   let mut config = BegoneConfig::default();
    debug!("Default Config: {:?}", config);
    get_roots(&mut config);
    debug!("Config After get_roots: {:?}", config);

    get_kill_patterns(&mut config);
    debug!("Config After get_kill_patterns: {:?}", config);
}
