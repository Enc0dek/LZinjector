use std::path::{PathBuf, Path};

pub fn get_file(name: String) -> Option<PathBuf>{
    let file = Path::new(&name);

    if file.exists(){
        Some(file.to_path_buf())
    }else{
        None
    }
}