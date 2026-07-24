use std::{fs, process::exit};

use crate::{agent::Agent::{Agent, AgentConfig, Memory, Ollama}, helper::Helper::CLI, tool::tools::Tools::insert_tools};

mod helper;
mod agent;
mod tool;


fn main() {
    let mut clargs = CLI::new();
    clargs.Parse_Args();
    if clargs.dbg{
        println!("{clargs:?}");
    }
   let mut agent = Agent::new(clargs.root_dir,Some(Box::new(Ollama::new(Some(clargs.url),Some(clargs.model)))), None,if let Some(x) = clargs.memory{Some(serde_json::from_str::<Memory>(&fs::read_to_string(x).unwrap()[..]).unwrap())}else{None},Some(clargs.steps),Some(AgentConfig::new(Some(clargs.steps), Some(clargs.token_limits.0), Some(clargs.token_limits.1), Some(clargs.token_limits.2), Some(clargs.temp))));
    if let Some(x) = clargs.sprompt{
        agent.memory.push_system(x);
    }else{
        eprintln!("System prompt is required for Modelling");
        exit(0);
    }

    insert_tools(&mut agent.tools);

    if clargs.dbg{
        println!("{}",agent);
    }   

    println!("Enter your request:");
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read input");
    let user_input = user_input.trim().to_string();

    match agent.run(user_input) {
        Ok(response) => {
            println!("Agent response:\n{}", response);
        }
        Err(e) => {
            eprintln!("Agent error: {}", e);
        }
    }




}
