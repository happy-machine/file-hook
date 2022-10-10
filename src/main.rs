use notify::{
  event::{CreateKind, EventKind},
  RecommendedWatcher, 
  RecursiveMode, 
  Watcher, 
  Config
};

use std::error::Error;
use std::collections::HashMap;
use std::{path::Path, path::PathBuf};

mod config;

fn post(full_path:String, type_string:String) -> Result<(), Box<dyn Error>> {
  let env = config::constants::get_env();
  let path = PathBuf::from(full_path);
  let target = path.file_name().unwrap();
  let mut map = HashMap::new();
  map.insert("path", target.to_str().unwrap());
  map.insert("type", &type_string);
  let client = reqwest::blocking::Client::new();
  let _res = client.post(env.endpoint)
      .json(&map)
      .send();
  Ok(())
}

fn main() {
  let env = config::constants::get_env();
  if let Err(e) = watch(env.request_sensor_path) {
      println!("error: {:?}", e)
  }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let env = config::constants::get_env();
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    if env.recursive_mode {
      watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    } else {
      watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;
    }
    let mut folder_history = vec![];

    for res in rx {
      if folder_history.len() > 50 {
        let folder_history_safe_length = folder_history.len().saturating_sub(50);
        folder_history.truncate(folder_history_safe_length);
      }
      match res {
        Ok(event) => {
          match event.kind {
            EventKind::Create(CreateKind::File) => {
              println!("new file: {} ", event.paths[0].display().to_string());   
              post(event.paths[0].display().to_string(), "file".to_string()); 
            }
            EventKind::Create(CreateKind::Folder) => {
              if !folder_history.contains(&event.paths[0].display().to_string()){
                println!("new folder: {} ", event.paths[0].display().to_string());  
                post(event.paths[0].display().to_string(), "folder".to_string());
                folder_history.push(event.paths[0].display().to_string());
              }
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