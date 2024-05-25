use std::error::Error;
use std::fs;
use std::process;

const CONFIG_FILE_PATH: &str = "./config/webpages.conf";

#[derive(Debug)]
pub struct Config {
    pub config : ConfigOptions,
}
#[derive(Debug)]
pub enum ConfigOptions {
    AddFile(String),
    ReadConfig,

}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 2 {
            return Ok(Config {config: ConfigOptions::ReadConfig});
        }
        else if args.len() == 2 {
            let file_path = args[1].clone();

            return Ok(Config {config : ConfigOptions::AddFile(  file_path) })
        }
        return Err("Incorrect amount of arguments");
    }
 
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.config {
        ConfigOptions::AddFile(file_path) => {
            add_file(file_path).unwrap_or_else(|err| {
                println!("Problem parsing file! {err}");
                process::exit(1);
            });
        },
        ConfigOptions::ReadConfig => {
            read_config().unwrap_or_else(|err| {
                println!("Problem parsing file! {err}");
                process::exit(1);
            });
        }
    }
    Ok(())
}
fn add_file(file_path : String) -> Result<(), Box<dyn Error>> {
    println!("attempting to parse file {file_path}");
    let file_contents = fs::read_to_string(file_path.clone())?;
    let data = fs::write(CONFIG_FILE_PATH, format!("{file_path}\n"));
    println!("successfully wrote file!");
    Ok(())
}
pub fn read_config() -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(CONFIG_FILE_PATH);
    let binding = file_contents?;
    let files = binding.split('\n').collect::<Vec<&str>>();
    println!("successfully read config, returned {:?}", files);
    Ok(())
}
