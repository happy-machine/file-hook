use std::env;

pub struct Env {
    pub recursive_mode: bool,
    pub request_sensor_path: String,
    pub endpoint: String
}

pub fn get_env() -> Env {
    return Env {
        recursive_mode: env::var("RECURSIVE_MODE").unwrap_or("false".to_string()).parse().unwrap(),
        request_sensor_path: env::var("REQUEST_SENSOR_PATH").unwrap_or("./".to_string()),
        endpoint: env::var("ENDPOINT")
            .expect("\nPlease set ENDPOINT environment variable to the endpoint you want to recieve file and folder names.\nTo set, execute \'export ENDPOINT=\"myapi.com/ExamplefileSensorEndpoint\"\'\n")
    };
}




