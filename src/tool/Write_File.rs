pub mod write_file{
    use std::{fs::write, path::PathBuf};

use crate::tool::{Create_Dir::create_dir::create_dir, tools::Tools::Tool};

    
    pub fn write_file(cwd:&PathBuf,target: &str,content:&str) -> String{
    let target_path = cwd.join(PathBuf::from(&target)).clean();

       if !target_path.starts_with(&cwd){
            return format!("Path {} does not Exist!",&target);
        }
        if !target_path.exists(){
            let k = create_dir(cwd, &target[..]);
            if !k.starts_with("Created dir"){
                return k;
            }
        }
        if target_path.is_dir(){
          return format!("{} is not a File!",target);
        }
        
        write(target_path, content).unwrap();
        format!("File {} written to",target)
    }

    struct write_file{
        cwd:PathBuf
    }

    impl write_file{
        pub fn new(cwd:PathBuf) -> Self{
            Self { cwd }
        }
    }

    impl Tool for write_file{

        fn name(&self) -> &str {
            "write_file"
        }
        fn description(&self) -> &'static str {
            "Creates the file with file contents provided alongside creation of intermediary directories in the path,simiar to tool create_file"
        }
        fn execute(
        &self,
        args: serde_json::Value,
        ) -> anyhow::Result<String>
        {
            let tgt = args.get("path").and_then(serde_json::Value::as_str).ok_or("Missing path")?;
            let content = args.get("content").and_then(serde_json::Value::as_str).ok_or("Missing content")?;
            Ok(write_file(&self.cwd, tgt,content))
        }

    }

}