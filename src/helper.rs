pub mod Helper{
    use std::path::{Path, PathBuf};

    use std::env;
    use std::process::exit;
    
    pub const DBG_STR:&str = "";
    pub const T_MIN:usize = 0;
    pub const T_MAX:usize = 5000;
    pub const URL:&str = "http://localhost:11434";

    pub fn Help(){
        println!("{DBG_STR}");
        exit(0);
    }


    #[derive(Debug)]
    pub struct CLI{
        pub dbg: bool,
        pub url: String,
        pub token_limits:(usize,usize),
        pub root_dir: PathBuf
    }


    impl CLI{
        pub fn new() -> Self{
            Self { dbg: false, url: URL.to_string(), token_limits: (T_MIN,T_MAX),root_dir:env::current_dir().unwrap()}
        }

        pub fn Parse_Args(&mut self){
            let mut args = env::args().skip(1).collect::<Vec<String>>();
            for i in &mut args{
                if i == "-d" || i == "--debug"{
                    self.dbg = true;
                } else if i == "-h" || i == "--help"{
                    Help();
                } else if i.starts_with("--url=") || i.starts_with("-u="){
                    self.url= i.split_off(i.find("=").unwrap());
                } else if i.starts_with("--min="){
                    self.token_limits.0 = i.split_off(i.find("=").unwrap()).parse::<usize>().expect("Min Tokens can be a non negative usize"); 
                } else if i.starts_with("max="){
                    self.token_limits.1 = i.split_off(i.find("=").unwrap()).parse::<usize>().expect("Max Tokens can be a non negative usize,for unbounded limit use --max=0"); 
                    if self.token_limits.1 == 0{
                        self.token_limits.1 = usize::MAX;
                    }
                } else if i.starts_with("--root=") || i.starts_with("--idir"){
                    self.root_dir= PathBuf::from(&i.split_off(i.find("=").unwrap())[1..]).canonicalize().unwrap();
                }else{

                    Help();
                }
            }
        }



    }



}