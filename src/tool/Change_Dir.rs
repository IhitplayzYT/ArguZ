pub mod change_dir{
    use std::path::PathBuf;

use crate::tool::tools::Tools::Tool;


    pub fn change_dir(cwd:&mut PathBuf,target:&str) -> String{
        let mut target_path = cwd.join(PathBuf::from(&target)).canonicalize().unwrap();
        if !target_path.starts_with(&cwd) || !target_path.exists(){
            return format!("Path {} does not Exist!",&target);
        }
        if target_path.is_file(){
            target_path = target_path.parent().ok_or("No parent dir").to_path_buf();
        }

        
        match std::env::set_current_dir(target_path){
            Ok(_) => {cwd = &mut target_path;format!("Changed working directory: {}",target_path)},
            _ => format!("Failed to change working directory")
        }
    }

    pub struct cd{
        cwd:PathBuf
    }

    impl cd {
        pub fn new(cwd:PathBuf) -> Self{
            Self { cwd }
        }
    }

impl Tool for cd{
fn name(&self) -> &str {
    "change_dir"
}
fn description(&self) -> &'static str {
    "changes the cwd incase path is to a file it changes to the file's parent directory"
}
fn execute(
        &mut self,
        args: serde_json::Value,
    ) -> anyhow::Result<String>
    {
        let tgt = args.get("path").and_then(serde_json::Value::as_str).ok_or("Missing path")?;
        Ok(change_dir(&mut self.cwd, tgt))
    }


}


}