use std::{fs, process::exit};

use crate::{agent::Agent::{Agent, AgentConfig, Memory, Ollama}, helper::Helper::CLI, tool::tools::Tools::{insert_tools, ToolRegistry}};

mod helper;
mod agent;
mod tool;

fn generate_tool_guide(tools: &ToolRegistry) -> String {
    let tool_names = tools.get_all();
    format!(
        "You have access to the following tools: {}\n\n\
        When you need to use a tool, respond with a JSON object in this format:\n\
        {{\n\
            \"type\": \"tool\",\n\
            \"name\": \"tool_name\",\n\
            \"arguments\": {{\n\
                \"param1\": \"value1\",\n\
                \"param2\": \"value2\"\n\
            }}\n\
        }}\n\n\
        When you have completed the task and have a final answer for the user, respond with:\n\
        {{\n\
            \"type\": \"final\",\n\
            \"content\": \"your final response here\"\n\
        }}",
        tool_names
    )
}

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

    // Add tool guide to system prompt
    let tool_guide = generate_tool_guide(&agent.tools);
    agent.memory.push_system(tool_guide);

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
