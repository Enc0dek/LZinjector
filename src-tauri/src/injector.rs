use std::path::PathBuf;
use dll_syringe::{Syringe, process::OwnedProcess};

pub fn inject_payload(process: String, payload: &PathBuf) -> Result<i32, String>{

    if let Some(tarjet_process) = OwnedProcess::find_first_by_name(process){
        let syringe = Syringe::for_process(tarjet_process);

        match syringe.inject(payload){
            Ok(_) => {
                Ok(0)
            },
            Err(e) => {
                let err_msg = format!("Failed to inject Dll: {:?}", e);
                Err(err_msg)
            }
        }
    }else{
        Err("Process Not found".to_string())
    }

}