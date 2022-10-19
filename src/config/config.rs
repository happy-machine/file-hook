use std::collections::HashMap;
use serde_yaml;
 use std::error::Error;

pub fn yaml() -> Result<String> {
    let f = std::fs::File::open("something.yaml")?;
    let data: serde_yaml::Value = serde_yaml::from_reader(f)?;
    match data {
        Ok(stuff): println!("{:?}", return stuff),
        Err(e): println!("{}", e)
    }
    // data["foo"]["bar"]
    //     .as_str()
    //     .map(|s| println!("{:?}", s));
// println!("{:?}", data["pipeline_name"].as_str())
}
