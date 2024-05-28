use std::env;
use std::process;
use obsidian_site::Config; 
// TODO: 
// - Start parsing files to html output when they are in the config directory
//      - figure out file structure
fn main() {
    println!("{}", std::env::current_dir().unwrap().display());

    let args: Vec<String> = env::args().collect();
    let mut configuration = Config::build(&args).unwrap_or_else(|err| {
        dbg!("Problem parsing file! {err}");
        process::exit(1);
    });
    
    // println!(sdfsdfsdf"Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = configuration.run() {
        dbg!("Problem parsing file! {e}");
        process::exit(1);
 
    }

}


