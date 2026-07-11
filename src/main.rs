use crate::helper::Helper::CLI;

mod helper;
mod tools;

fn main() {
    let mut clargs = CLI::new();
    clargs.Pare_Args();
    if clargs.dbg{
        println!("{clargs:?}");
    }


}
