// use std::io::{Read, Write};
// use std::path::{Path, PathBuf};
// use std::fs::File;
// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Config {
//     payload_path: PathBuf,
//     process: String,
//     custom_target: bool,
// }

// pub fn config_init(name: String){
//     let file_path = Path::new(&name);

//     if !file_path.exists(){
//         File::create(file_path).unwrap();
//     }

// }

// pub fn read_config(name: String) -> Option<Config> {
//     let file_path = Path::new(&name);
//     let mut buf = String::new();

//     if file_path.exists() {
//         let mut file = File::open(file_path).unwrap();
//         file.read_to_string(&mut buf).unwrap();

//         match serde_json::from_str(&buf) {
//             Ok(config) => Some(config),
//             Err(_) => None,
//         }
//     } else {
//         None
//     }
// }

// pub fn save_config(file_path: &Path, payload_path: PathBuf, process: String, custom_target: bool) -> Result<i32, String> {
//     let config = Config {
//         payload_path,
//         process,
//         custom_target,
//     };

//     let real_file = match file_path.canonicalize() {
//         Ok(path) => path,
//         Err(e) => return Err(format!("Failed to canonicalize file path: {}", e)),
//     };

//     if real_file.exists() {
//         println!("{:?}", real_file);
//         let mut file = match File::open(&real_file) {
//             Ok(file) => file,
//             Err(e) => return Err(format!("Failed to open file: {}", e)),
//         };

//         match serde_json::to_string(&config) {
//             Ok(data) => {
//                 match file.write(data.as_bytes()) {
//                     Ok(_) => Ok(0),
//                     Err(e) => {
//                         let errormsg = format!("Failed to save the config: {}, {}", e, data);
//                         Err(errormsg)
//                     }
//                 }
//             }
//             Err(_e) => {
//                 Err("Invalid config".to_string())
//             }
//         }
//     } else {
//         Err("File does not exist".to_string())
//     }
// }