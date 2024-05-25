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
fn add_file(new_file_path : String) -> Result<(), Box<dyn Error>> {
    let mut out_string : String = String::from("");
    // refactor this to convert the file into a list of strings, split by newlines (handle indented tabs later)
    println!("successfully wrote file!");
    let config_file_contents = fs::read_to_string(&CONFIG_FILE_PATH)?;
    let mut config_contents = config_file_contents.split('\n').collect::<Vec<&str>>();

    let config_file_contents = fs::read_to_string(&new_file_path)?;
    config_contents.push(&new_file_path);
    config_contents.sort();
    config_contents.dedup();
    config_contents.into_iter().for_each(|current_file| {

        out_string += &format!("{current_file}\n");
    });
    fs::write(&CONFIG_FILE_PATH, out_string);
    Ok(())
}
pub fn read_config() -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(CONFIG_FILE_PATH);
    let binding = file_contents?;
    let files = binding.split('\n').collect::<Vec<&str>>();
    println!("successfully read config, returned {:?}", files);
    Ok(())
}
