use notify::{
    event::{CreateKind, Event, EventKind},
    RecursiveMode, Result, Watcher,
};
use std::time;
use std::path::PathBuf;
use std::{env, sync::atomic::Ordering};
use std::{path::Path, sync::Arc};
use std::{sync::atomic::AtomicI32};
use std::collections::HashMap;
use ticktock::Timer;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long, action)]
  wait: bool,
  #[clap(short, long, value_parser, default_value_t = 20)]
  seconds: u8,
}

fn post(full_path:String, type_string:String, ENDPOINT: String) -> Result<()> {
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

fn main() -> Result<()> {
  let args = Args::parse();
  let mut ENDPOINT: String = "".to_string();
    if args.wait == false {
      ENDPOINT = env::var("ENDPOINT").expect("\nPlease set ENDPOINT environment variable to the endpoint you want to recieve file and folder names.\nTo set, execute \'export ENDPOINT=\"myapi.com/ExamplefileSensorEndpoint\"\'\n");
    }
  let FILE_SENSOR_DELAY = std::env::var("FILE_SENSOR_DELAY").unwrap_or("60".to_string());
  let REQUEST_SENSOR_PATH =
  env::var("REQUEST_SENSOR_PATH").unwrap_or("./".to_string());

  let file_sensor_delay: i32 = FILE_SENSOR_DELAY.parse().unwrap();
  let now = time::Instant::now();
  let mut heartbeat= Timer::apply(
      |_, count| {
          *count += 1;
          *count
      },
      0,
  )
  .every(time::Duration::from_millis(1000))
  .start(now);
  if args.wait {
    println!("Waiting for files/folders...");
  } else {
    println!("Waiting for files/folders to send requests...");
  }

  let file_count = Arc::new(AtomicI32::new(0));
  let mut watcher = notify::recommended_watcher({
  let file_count = Arc::clone(&file_count);
    move |res: Result<Event>| {
      match res {
        Ok(event) => {
          match event.kind {
            EventKind::Create(CreateKind::File) => {   
              file_count.fetch_add(1, Ordering::AcqRel);
              if args.wait != true {
                post(event.paths[0].display().to_string(), "file".to_string(), ENDPOINT.clone());
              }
              println!("new file: {} ", event.paths[0].display().to_string())   
            }
            EventKind::Create(CreateKind::Folder) => {
              file_count.fetch_add(1, Ordering::AcqRel);
              if  args.wait != true {
                  post(event.paths[0].display().to_string(), "folder".to_string(), ENDPOINT.clone());
              }
              println!("new folder: {} ", event.paths[0].display().to_string())   
            }
            _ => { /* something else changed */ }
          }
        }
        Err(e) => {
          panic!("watch error: {:?}", e);
        }
      }
    }
  })?;

  watcher.watch(Path::new(&REQUEST_SENSOR_PATH), RecursiveMode::Recursive)?;
  let mut file_total = 0;
  loop {
    if args.wait == true {
    let now = time::Instant::now();
      if let Some(n) = heartbeat.update(now) {
        if file_total > 0 && n > file_sensor_delay  {
          println!("Exiting...");
          std::process::exit(101);
            
        } 
        if (file_count.load(Ordering::Acquire)) > file_total {
          println!("Waiting for {} seconds...", file_sensor_delay);
          heartbeat.set_value(0);
          file_total += 1;
        }
      }
    }
  }
}