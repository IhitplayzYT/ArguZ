pub mod print_wd{
    use std::path::PathBuf;

use crate::tool::tools::Tools::Tool;


pub fn print_wd(cwd:PathBuf) -> String{
    match std::env::current_dir().unwrap().strip_prefix(&cwd){
        Ok(x) => format!("Current WD: {}",x),
        _ => format!("CRITICAL Error!Abort!")
    }
}

pub struct pwd{
    cwd:PathBuf
}

impl pwd{
    pub fn new(cwd:PathBuf) -> Self{
        Self { cwd }
    }
}

impl Tool for pwd{


    fn name(&self) -> &str {
        "print_wd"
    }
    fn description(&self) -> &'static str {
        "Prints the current working directory"
    }
    fn execute(
        &self,
        _args: serde_json::Value,
    ) -> anyhow::Result<String>
    {
       anyhow::Ok(print_wd(self.cwd)) 
    }

}




}