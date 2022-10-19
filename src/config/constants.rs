use std::collections::HashMap;
use std::env;
use std::any::Any;

enum EnvTypeKind {
    BOOL,
    STR,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64
}


pub fn safe_env(key: String) -> Any {
    env::var("ENDPOINT")
        .expect("\nPlease set ENDPOINT environment variable to the endpoint you want to recieve file and folder names.\nTo set, execute \'export ENDPOINT=\"myapi.com/ExamplefileSensorEndpoint\"\'\n");
    let this_env: HashMap<&str, (EnvTypeKind, String)> = HashMap::from([
        ("RECURSIVE_MODE", (EnvTypeKind::BOOL, "false".to_string())),
        ("REQUEST_SENSOR_PATH", (EnvTypeKind::STR, "".to_string())),
        ("ENDPOINT", (EnvTypeKind::STR, "./".to_string()))
    ]);
    for (key_inner, val) in env::vars() {
        if this_env.contains_key(&key as &str){
            match this_env.get(&key as &str){
                //BOOL => return val.parse().unwrap_or((this_env.get(&key as &str)).1)
               BOOL => println!("boom"),
            }
            return val;
        } else if key == key_inner {
            return val;
        }
        println!("env var: {:?}", key)
    }
    return "random".to_string();
    // println!("test")
}

// env::var("ENDPOINT").expect("\nPlease set ENDPOINT environment variable to the endpoint you want to recieve file and folder names.\nTo set, execute \'export ENDPOINT=\"myapi.com/ExamplefileSensorEndpoint\"\'\n");
// let RECURSIVE_MODE =
//     env::var("RECURSIVE_MODE").unwrap_or("false".to_string())
    
pub struct Env2 {
    recursive_mode: bool,
    request_sensor_path: String,
    endpoint: String
}



