pub mod list_dir{
    use std::{fs, path::PathBuf};

use crate::tool::tools::Tools::{Tool, mode_to_string};

    

    pub fn list_dir(cwd:&PathBuf,target:&str) -> String{
        let mut ret = String::new();
        let target_path = cwd.join(PathBuf::from(&target)).canonicalize().unwrap();
        if !target_path.starts_with(&cwd) || !target_path.exists(){
            return format!("Path {} does not Exist!\n",&target);
        }
        fs::read_dir(target_path).unwrap().for_each(|x|{
            let content = x.unwrap();
            let fullpath = content.path();
            let relative = fullpath.strip_prefix(&cwd).unwrap();
            let meta = content.metadata().unwrap();
            let perm = meta.permissions();
            ret += &format!("{} {} {}B\n",relative.to_str().unwrap(),mode_to_string(perm.mode()),meta.len())
        });
        ret            
    }
pub struct ls{
    cwd:PathBuf,
}

impl ls{
    pub fn new(cwd:PathBuf) -> Self{
        Self{cwd}
    }
}

impl Tool for ls{
    fn name(&self) -> &str {
        "list_dir"
    }

    fn description(&self) -> &'static str {
        "List directory contents alongside some file metadata like permissionsand size"
    }

    fn execute(
        &self,
        args: serde_json::Value,
    ) -> anyhow::Result<String>
    {
        let tgt = args.get("path").and_then(serde_json::Value::as_str).ok_or("Missing path")?;
        Ok(list_dir(&self.cwd, tgt))
    }

}

}