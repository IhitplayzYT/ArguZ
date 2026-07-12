pub mod Agent{
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};


use crate::tool::tools::Tools::{Tool, ToolRegistry};


pub struct Agent {
    cwd:PathBuf,
    model: Box<dyn LLM>,
    tools: ToolRegistry,
    memory: Memory,
    steps: usize,    
    config: AgentConfig,
    state: AgentState,    
    cancelled: Arc<AtomicBool>,

}

#[derive(Clone)]
pub struct AgentConfig {
    pub max_steps: usize,
    pub max_context_tokens: usize,
    pub max_output_tokens: usize,
    pub temperature: f32,
}
impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_steps: 50,
            max_context_tokens: 100_000,
            max_output_tokens: 4096,
            temperature: 0.0,
        }
    }
}

impl Agent {

    pub fn new(
        cwd: PathBuf,
        model: Box<dyn LLM>,
    ) -> anyhow::Result<Self> {

        let cwd = cwd.canonicalize()?;

        Ok(Self {

            cwd: cwd.clone(),

            model,

            tools: ToolRegistry::new(cwd),

            memory: Memory::new(),

            config: AgentConfig::default(),

            state: AgentState::Idle,

            steps: 0,

            cancelled: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn register_tool<T: Tool + 'static>(
        &mut self,
        tool: T,
    ) {
        self.tools.register(tool);
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    pub fn cwd(&self) -> &PathBuf {
        &self.cwd
    }


}

pub struct Memory {
    messages: Vec<Message>,
}

impl Memory {

    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn push_user(
        &mut self,
        text: String,
    ) {
        self.messages.push(Message::User(text));
    }

    pub fn push_tool(
        &mut self,
        tool: String,
        output: String,
    ) {
        self.messages.push(
            Message::Tool(tool, output)
        );
    }

    pub fn push_assistant(
        &mut self,
        text: String,
    ) {
        self.messages.push(
            Message::Assistant(text)
        );
    }

}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Idle,
    Thinking,
    ExecutingTool,
    Finished,
    Cancelled,
}

#[derive(Clone)]
pub enum Message {
    System(String),
    User(String),
    Assistant(String),
    Tool(String,String),
}

pub trait LLM {
    fn complete(
        &mut self,
        memory: &Memory,
    ) -> anyhow::Result<ModelResponse>;
}
pub struct Ollama {
    pub endpoint: String,
    pub model: String,
}
impl LLM for Ollama {

    fn complete(
        &mut self,
        memory: &Memory,
    ) -> anyhow::Result<ModelResponse> {


        // Build request

        // POST http://localhost:11434/api/chat

        // Parse response

        todo!()
    }

}

pub enum ModelResponse {

    ToolCall {
        name: String,
        arguments: serde_json::Value,
    },

    Final(String),
}

}