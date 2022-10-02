use notify::{
  event::{CreateKind, EventKind},
  RecommendedWatcher, 
  RecursiveMode, 
  Watcher, 
  Config
};

use std::error::Error;
use std::env;
use std::collections::HashMap;
use std::{path::Path, path::PathBuf};

fn post(full_path:String, type_string:String) -> Result<(), Box<dyn Error>> {
  let mut ENDPOINT: String = "".to_string();
  ENDPOINT = env::var("ENDPOINT").expect("\nPlease set ENDPOINT environment variable to the endpoint you want to recieve file and folder names.\nTo set, execute \'export ENDPOINT=\"myapi.com/ExamplefileSensorEndpoint\"\'\n");
  let path = PathBuf::from(full_path);
  let target = path.file_name().unwrap();
  let mut map = HashMap::new();
  map.insert("path", target.to_str().unwrap());
  map.insert("type", &type_string);
  let client = reqwest::blocking::Client::new();
  let _res = client.post(ENDPOINT)
      .json(&map)
      .send();
  Ok(())
}

fn main() {
  let REQUEST_SENSOR_PATH =
    env::var("REQUEST_SENSOR_PATH").unwrap_or("./".to_string());
  if let Err(e) = watch(REQUEST_SENSOR_PATH) {
      println!("error: {:?}", e)
  }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let RECURSIVE_MODE =
    env::var("RECURSIVE_MODE").unwrap_or("false".to_string());
    let recursive_mode: bool = RECURSIVE_MODE.parse().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    if recursive_mode {
      watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    } else {
      watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    }

    for res in rx {
      match res {
        Ok(event) => {
          match event.kind {
            EventKind::Create(CreateKind::File) => {
              println!("new file: {} ", event.paths[0].display().to_string());   
              post(event.paths[0].display().to_string(), "file".to_string()); 
            }
            EventKind::Create(CreateKind::Folder) => {
              println!("new folder: {} ", event.paths[0].display().to_string());  
              post(event.paths[0].display().to_string(), "folder".to_string());
            }
            _ => { /* something else changed */ }
          }
        }
        Err(e) => {
          panic!("watch error: {:?}", e);
        }
      }
    }
  Ok(())
}