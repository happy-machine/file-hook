use std::any::Any;
// use notify::{
//   event::{CreateKind, EventKind},
//   RecommendedWatcher, 
//   RecursiveMode, 
//   Watcher, 
//   Config
// };

// use std::error::Error;
// use std::collections::HashMap;
// use std::{path::Path, path::PathBuf};

mod config;

// fn post(full_path:String, type_string:String) -> Result<(), Box<dyn Error>> {
//   let env = config::constants::get_env();
//   let path = PathBuf::from(full_path);
//   let target = path.file_name().unwrap();
//   let mut map = HashMap::new();
//   map.insert("path", target.to_str().unwrap());
//   map.insert("type", &type_string);
//   let client = reqwest::blocking::Client::new();
//   let _res = client.post(env.endpoint)
//       .json(&map)
//       .send();
//   Ok(())
// }

fn main() {
  let test: dyn Any = config::constants::safe_env("RECURSIVE_MODE".to_string());
  //println!("test {:?}", test.1)
  // let env = config::constants::get_env();
  // if let Err(e) = watch(env.request_sensor_path) {
  //     println!("error: {:?}", e)
  // }
}

