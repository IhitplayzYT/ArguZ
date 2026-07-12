pub mod create_file{
    use std::{fs, path::PathBuf};

use crate::tool::{Create_Dir::create_dir::create_dir, tools::Tools::Tool};

    // Creates the dir and file if not exists and writes content to the file can be used for rewritting a file as well[Conditions managed internally]
    pub fn create_file(cwd:&PathBuf,target:&str,content:&str) -> String{
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
        let k = match fs::write(target_path, content){
            Err(_) => "Dir path created but something went wrong",
            _ => ""
        };
        if !k.is_empty(){
            return k.to_string();
        }
        format!("File {} created and written to",target)
    }

    struct touch{
        cwd:PathBuf
    }

    impl touch{
        pub fn new(cwd:PathBuf) -> Self{
            Self { cwd }
        }
    }

    impl Tool for touch{

        fn name(&self) -> &str {
            "create_file"
        }
        fn description(&self) -> &'static str {
            "Creates the file with file contents provided alongside creation of intermediary directories in the path,"
        }
        fn execute(
        &self,
        args: serde_json::Value,
        ) -> anyhow::Result<String>
        {
            let tgt = args.get("path").and_then(serde_json::Value::as_str).ok_or("Missing path")?;
            let content = args.get("content").and_then(serde_json::Value::as_str).ok_or("Missing content")?;
            Ok(create_file(&self.cwd, tgt,content))
        }

    }





}