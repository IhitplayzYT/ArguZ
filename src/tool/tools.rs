pub mod Tools{
    use std::{cmp::Ordering, collections::HashMap, f32::consts::E, fmt::Debug, fs::{self, DirEntry, FileType, Permissions, write}, os::unix::fs::PermissionsExt, path::PathBuf, process::Command};
use path_clean::PathClean;


pub trait Tool {
    fn name(&self) -> &str;

    fn execute(
        &mut self,
        args: serde_json::Value,
    ) -> anyhow::Result<String>;
    fn description(&self) -> &'static str;
}


pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register<T: Tool + 'static>(
        &mut self,
        tool: T,
    ) {
        self.tools
            .insert(tool.name().to_string(), Box::new(tool));
    }

    pub fn execute(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> anyhow::Result<String> {

        let tool = self
            .tools
            .get(name)
            .ok_or(anyhow::anyhow!("Unknown tool"))?;

        tool.execute(args)
    }
}


    pub fn mode_to_string(mode: u32) -> String {
        let mut s = String::with_capacity(10);
        s.push(match mode & 0o170000 {
            0o100000 => '-', // Regular file
            0o040000 => 'd', // Directory
            0o120000 => 'l', // Symlink
            0o020000 => 'c', // Character device
            0o060000 => 'b', // Block device
            0o010000 => 'p', // FIFO/Pipe
            0o140000 => 's', // Socket
            _ => '?',
        });
        const BITS: &[(u32, char)] = &[
            (0o400, 'r'), (0o200, 'w'), (0o100, 'x'),
            (0o040, 'r'), (0o020, 'w'), (0o010, 'x'),
            (0o004, 'r'), (0o002, 'w'), (0o001, 'x'),
        ];
        for &(bit, ch) in BITS {
            s.push(if mode & bit != 0 { ch } else { '-' });
        }
        if mode & 0o4000 != 0 {
            s.replace_range(3..4, if mode & 0o100 != 0 { "s" } else { "S" });
        }
        if mode & 0o2000 != 0 {
            s.replace_range(6..7, if mode & 0o010 != 0 { "s" } else { "S" });
        }
        if mode & 0o1000 != 0 {
            s.replace_range(9..10, if mode & 0o001 != 0 { "t" } else { "T" });
        }
        s
    }

    pub fn estimate_tokens(s: &str) -> usize {
        (s.len() + 3) / 4
    }

    pub fn truncate_head_tail(s: &str, head: usize, tail: usize) -> String {
        let lines: Vec<_> = s.lines().collect();

        if lines.len() <= head + tail {
            return lines.join("\n");
        }

        let mut out = String::new();

        out.push_str(&lines[..head].join("\n"));
        out.push_str("\n\n... OUTPUT TRUNCATED ...\n\n");
        out.push_str(&lines[lines.len()-tail..].join("\n"));

        out
    }


}