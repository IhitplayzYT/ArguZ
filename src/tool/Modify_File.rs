pub mod modify_file{
    use std::path::PathBuf;

use serde::Deserialize;

use crate::tool::{Cat_File::cat_file::cat_file, Create_File::create_file::create_file, Write_File::write_file::write_file, tools::Tools::Tool};

    pub fn modify_file(cwd:&PathBuf,target:&str,mut changes: Vec<(usize,usize,&str)>) -> String{
        
    let target_path = cwd.join(PathBuf::from(&target)).clean();
        if !target_path.exists(){
            let ret = create_file(cwd, &target[..], "");
            if !ret.ends_with("created and written to"){
                return ret;
            }
        }
        changes.sort_by(|x,y|{y.0.cmp(&x.0)});

        let mut buff = cat_file(cwd, &target[..]);
        println!("{buff}");
        if buff.ends_with(" does not Exist!") || buff.ends_with(" is not a file"){
            return buff;
        }
        let (mut removals,mut additions) = ((0,0),(0,0));
        let flen = buff.len();
        for (strt,end,content) in &changes{
            if *end > flen{
                continue;
            }
            removals.0 += end-strt;
            additions.0 += content.len();
            removals.1 = buff[*strt..*end].chars().fold(0, |acc,x| acc+if x == '\n'{1}else{0});
            additions.1 = content.chars().fold(0, |acc,x| acc+if x == '\n'{1}else{0});

            buff.replace_range(*strt..*end, content);
        }

        let k = write_file(cwd, target, &buff[..]);
        if !k.ends_with(" written to"){
            return k;
        }

        format!("File {} ,Lines -> +{} and -{} ,Chars => +{} and -{} ,modified",target,additions.1,removals.1,additions.0,removals.0)
    }


    pub struct edit{
        cwd:PathBuf
    }

    #[derive(Deserialize)]
    
    struct Changes{
        path:String,
        changes: Vec<(usize,usize,&'static str)>
    }


    impl edit{
        pub fn new(cwd:PathBuf) -> Self{
            Self { cwd }
        }
    }

    impl Tool for edit{
        fn name(&self) -> &str {
            "modify_file"
        }

        fn description(&self) -> &'static str {
            "Used to edt sections of a file using start,end indexes and replacement content,assume that the index values are internally managed to avoid offset changeing after each individual replacement"
        }
        fn execute(
        &self,
        args: serde_json::Value,
        ) -> anyhow::Result<String>
        {
            let mut params: Changes = serde_json::from_value(args)?;
            Ok(modify_file(&self.cwd, &params.path,params.changes))
        }
        

    }





}