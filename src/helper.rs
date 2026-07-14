pub mod Helper{
    use std::path::{Path, PathBuf};

    use std::env;
    use std::process::exit;

use serde::de::value::UsizeDeserializer;

use crate::agent::Agent::Memory;
    
    pub const DBG_STR:&str = "";
    pub const T_MIN:usize = 0;
    pub const T_MAX:usize = 100000;
    pub const TO_MAX:usize = 8192;
    
    pub const END_POINT:&str = "http://localhost:11434";
    pub const MODEL:&str = "llama3.2";
    pub fn Help(){
        println!("{DBG_STR}");
        exit(0);
    }


    #[derive(Debug)]
    pub struct CLI{
        pub dbg: bool,
        pub url: String,
        pub token_limits:(usize,usize,usize), // (min_context,max_context,max_output)
        pub root_dir: PathBuf,
        pub model: String,
        pub steps:usize,
        pub memory:Option<String>,
        pub temp:f32,
        pub sprompt:Option<String>,
    }


    impl CLI{
        pub fn new() -> Self{
            Self { dbg: false, url: END_POINT.to_string(), token_limits: (T_MIN,T_MAX,TO_MAX),root_dir:env::current_dir().unwrap(),model:MODEL.to_string(),steps:10,memory:None,temp:0.4,sprompt:None}
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
                    self.token_limits.0 = i.split_off(i.find("=").unwrap()).parse::<usize>().expect("Min Tokens is a non negative usize"); 
                } else if i.starts_with("--max="){
                    self.token_limits.1 = i.split_off(i.find("=").unwrap()).parse::<usize>().expect("Max Tokens is a non negative usize,for unbounded limit use --max=0"); 
                    if self.token_limits.1 == 0{
                        self.token_limits.1 = usize::MAX;
                    }
                } else if i.starts_with("--maxout="){
                    self.token_limits.2 = i.split_off(i.find("=").unwrap()).parse::<usize>().expect("Max Output Tokens is a non negative usize"); 
                }
                 else if i.starts_with("--root=") || i.starts_with("--idir"){
                    self.root_dir= PathBuf::from(&i.split_off(i.find("=").unwrap())[1..]).canonicalize().unwrap();
                }else if i.starts_with("--model="){
                    self.model = i.split_off(i.find("=").unwrap()+1);
                }else if i.starts_with("--steps=") || i.starts_with("-s="){ 
                    self.steps = (i.split_off(i.find("=").unwrap()+1)).parse().expect("Steps has to be usize");
                }else if i.starts_with("--memory="){ 
                    self.memory = Some(i.split_off(i.find("=").unwrap()+1));
                }else if i.starts_with("--temp=") || i.starts_with("-t="){ 
                    self.temp = (i.split_off(i.find("=").unwrap()+1)).parse().expect("Temp has to be f32 lying inbetween 0.0 - 2.0");
                }else if i.starts_with("--sysprompt=") || i.starts_with("--prompt="){ 
                    self.sprompt = Some(i.split_off(i.find("=").unwrap() + 1));
                }
                else{

                    Help();
                }
            }
        }



    }



}