use std::error::Error;
use std::fs;
use std::fs::read_to_string;
use std::io::Write;
use std::process;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

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
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let files : Vec<String> = match &self.config {
            ConfigOptions::AddFile(file_path) => {
                dbg!(&file_path);
                add_file(file_path.to_string()).unwrap_or_else(|err| {
                    dbg!(format!("Problem parsing file! {err}"));
                    process::exit(1);
                })
            },
            ConfigOptions::ReadConfig => {
                read_config().unwrap_or_else(|err| {
                    dbg!("Problem parsing file! {err}");
                    process::exit(1);
                })
            }
        };
        serve_files(files);
        Ok(())
    }
}

fn add_file(new_file_path : String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut out_string : String = String::from("");
    // refactor this to convert the file into a list of strings, split by newlines (handle indented tabs later)
    let file = File::open(Path::new(&CONFIG_FILE_PATH)).unwrap();
    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    lines.remove(0);
    lines.push(new_file_path.clone());
    lines.sort();
    lines.dedup();
    lines.clone().into_iter().for_each(|current_file| {

        out_string += &format!("{current_file}\n");
    });
    // fs::create_dir("/some/dir")?
    fs::write(&CONFIG_FILE_PATH, out_string)?;
    println!("successfully added file!{}", &new_file_path);
    Ok(lines)
}

fn serve_files(file_paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    dbg!(&file_paths);
    file_paths.iter().for_each(|file_path| {
        // ERROR: 
        dbg!(file_path);
        let file_name = Path::new(&file_path).file_stem().unwrap().to_str().unwrap();
        
        // Create the directory path
        let page_dir_string = format!("./pages/{}", file_name);
        let page_directory = Path::new(&page_dir_string);
        
        // Create the full path to the new file
        let page_file_string = format!("{}/{}.md", page_dir_string, file_name);
        let page_file = Path::new(&page_file_string);
        
        // Create the directory if it does not exist
        let _ = fs::create_dir_all(page_directory).unwrap_or_else(|err| println!("Failed to create directory: {}", err));
        
        println!("Page Dir: {:?}\nPage File: {:?}", &page_directory, &page_file);
        
        
        // Create the file
        let curr_file_path_string = String::from(&format!("{file_path}"));
        dbg!(&curr_file_path_string);
        let curr_file_path = Path::new(&curr_file_path_string);
        let curr_file = File::create(page_file).unwrap();
        let curr_file_contents = fs::read_to_string(curr_file_path).unwrap();
        fs::write(&page_file, curr_file_contents);
    });


    Ok(())
}
fn clean_file_path(new_file_path : String) -> String {
    let mut out : String = String::from("");
    let count =     new_file_path.matches('/').count();

    if count == 0{ 
        out = format!("/{}", new_file_path) 
    }
    String::from(out)
}
pub fn read_config() -> Result<Vec<String>, Box<dyn Error>> {
    let file_contents = fs::read_to_string(&CONFIG_FILE_PATH)?;
    let file = File::open(Path::new(&CONFIG_FILE_PATH)).unwrap();
    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    lines.remove(0);
    println!("FOUND CONFIG LINES {:?}", lines);
    // println!("successfully read config, returned {:?}", lines);
    Ok(lines.to_vec())
}
