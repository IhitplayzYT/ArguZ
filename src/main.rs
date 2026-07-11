use std;

use crate::{helper::Helper::CLI, tools::Tools::{cat_file, create_dir, create_file, list_dir, modify_file}};

mod helper;
mod tools;


fn main() {
    let mut clargs = CLI::new();
    clargs.Pare_Args();
    if clargs.dbg{
        println!("{clargs:?}");
    }
    let root = clargs.root_dir;
    let content ="gibberish";
    println!("{}",modify_file(&root, ".",vec![(0,2,"asuib")]));
    println!("{}",modify_file(&root, "../..",vec![(0,2,"asuib")]));
    println!("{}",cat_file(&root, "l/n/5.txt"));
    println!("{}",modify_file(&root, "l/n/5.txt",vec![(0,5,"bye bye"),(6,11,"sekai"),(17,21,"kjasbku")]));
}
