pub mod remove_dir{
    use std::{fs, path::PathBuf};

use crate::tool::tools::Tools::Tool;



    // Can delete files as well
    pub fn remove_dir(cwd:&PathBuf, target: &str) -> String {
        // Resolve relative to the sandbox root, not the process cwd.
        let joined = cwd.join(&target);
        let target_path = match joined.canonicalize() {
            Ok(p) => p,
            Err(_) => return format!("Path '{}' does not Exist!", target),
        };
        if !target_path.starts_with(&cwd) {
            return format!("Path {} does not Exist!",&target);

        }
        let result = if target_path.is_dir() {
            fs::remove_dir_all(&target_path)
        } else {
            fs::remove_file(&target_path)
        };
        match result {
            Ok(_) => format!("Removed '{}'", target),
            Err(e) => format!("Failed to remove '{}': {}", target, e),
        }
    }

    struct rm{
        cwd:PathBuf
    }

    impl rm{
        pub fn new(cwd:PathBuf) -> Self{
            Self { cwd }
        }
    }

    impl Tool for rm{
        fn name(&self) -> &str {
            "remove_dir"
        }

        fn description(&self) -> &'static str {
            "Similar to Linux rm -rf used to delete files and directories"
        }
        fn execute(
        &self,
        args: serde_json::Value,
        ) -> anyhow::Result<String>
        {
            let tgt = args.get("path").and_then(serde_json::Value::as_str).ok_or("Missing path")?;
            Ok(remove_dir(&self.cwd, tgt))
        }
        

    }



}