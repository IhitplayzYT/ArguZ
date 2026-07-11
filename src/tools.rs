pub mod Tools{
    use std::{f32::consts::E, fmt::Debug, fs::{self, DirEntry, FileType, Permissions}, os::unix::fs::PermissionsExt, path::PathBuf, process::Command};


    pub fn list_dir(cwd:PathBuf,target:String) -> String{
        let mut ret = String::new();
        let target_path = PathBuf::from(&target).canonicalize().unwrap();
        if !target_path.starts_with(&cwd) || !target_path.exists(){
            return format!("Path {} does not Exist!",&target);
        }
        fs::read_dir(target_path).unwrap().for_each(|x|{
            let content = x.unwrap();
            let fullpath = content.path();
            let relative = fullpath.strip_prefix(&cwd).unwrap();
            let meta = content.metadata().unwrap();
            let perm = meta.permissions();
            ret += &format!("{} {} {}B\n",relative.to_str().unwrap(),mode_to_string(perm.mode()),meta.len())
        });
        ret            
    }

    pub fn cat_file(cwd:PathBuf,target:String) -> String{
        let target_path = PathBuf::from(&target).canonicalize().unwrap();
        if !target_path.starts_with(&cwd) || !target_path.exists(){
            return format!("Path {} does not Exist!",&target);
        }
        if target_path.is_dir(){
            return format!("Path {} is not a file",&target);
        }
        fs::read_to_string(target_path).unwrap()
    }

    pub fn create_dir(cwd:PathBuf,target:String) -> String{
        let mut target_path = PathBuf::from(&target).canonicalize().unwrap();
        let temp = target_path.clone();
        let rel_path = temp.as_path().strip_prefix(&cwd).unwrap().to_str().unwrap();
        if !target_path.starts_with(&cwd){
            return format!("Path {} does not Exist!",&rel_path);
        }
        if target_path.exists(){
            return format!("Path {} already Exists!",&rel_path)
        }
        if target_path.is_file(){
            target_path = target_path.parent().unwrap().to_path_buf();
        }
        let k = match fs::create_dir_all(&target_path){
            Err(_) => &format!("Failed to create dir: {}",rel_path),
            _ => ""
        };
        if !k.is_empty(){
            return k.to_string();
        }
        format!("Created dir: {}",rel_path)
    }


    // Creates the dir and file if not exists and writes content to the file can be used for rewritting a file as well[Conditions managed internally]
    pub fn create_file(cwd:PathBuf,target:String,content:String) -> String{
        let target_path = PathBuf::from(&target).canonicalize().unwrap();
        let temp = target_path.clone();
        let rel_path = temp.as_path().strip_prefix(&cwd).unwrap().to_str().unwrap();
        if !target_path.starts_with(&cwd){
            return format!("Path {} does not Exist!",&target);
        }
        if !target_path.exists(){
            let k = create_dir(cwd, target);
            if !k.starts_with("Created dir"){
                return k;
            }
        }
        let k = match fs::write(target_path, content){
            Err(_) => "Dir path created but something went wrong",
            _ => ""
        };
        if !k.is_empty(){
            return k.to_string();
        }

        format!("File {} created and written to",rel_path)
    }

    pub fn modify_file(cwd:PathBuf,target:String,changes: Vec<(usize,usize,String)>) -> String{
        let ret = create_file(cwd.clone(), target.clone(), "".to_string());
        if !ret.ends_with("created and written to"){
            return ret;
        }
        let target_path = PathBuf::from(&target).canonicalize().unwrap();
        let rel_path = target_path.as_path().strip_prefix(&cwd).unwrap().to_str().unwrap();
        let mut buff = cat_file(cwd, target);
        if buff.ends_with(" does not Exist!") || buff.ends_with(" is not a file"){
            return buff;
        }
        let (mut removals,mut additions) = ((0,0),(0,0));
        for (strt,end,content) in &changes{
            removals.0 += end-strt;
            additions.0 += content.len();
            removals.1 = buff[*strt..*end].chars().fold(0, |acc,x| acc+if x == '\n'{1}else{0});
            additions.1 = content.chars().fold(0, |acc,x| acc+if x == '\n'{1}else{0});
            buff.replace_range(*strt..*end, content);
        }
        format!("File {} ,Lines -> +{} and -{} ,Chars => +{} and -{} ,modified",rel_path,additions.1,removals.1,additions.0,removals.0)
    }



    pub fn remove_dir(cwd: PathBuf, target: String) -> String {
        // Resolve relative to the sandbox root, not the process cwd.
        let joined = cwd.join(&target);
        let target_path = match joined.canonicalize() {
            Ok(p) => p,
            Err(_) => return format!("Path '{}' does not Exist!", target),
        };
        if !target_path.starts_with(&cwd) {
            return format!("Path {} does not Exist!",&target);

        }
        let result = if target_path.is_dir() {
            fs::remove_dir_all(&target_path)
        } else {
            fs::remove_file(&target_path)
        };
        match result {
            Ok(_) => format!("Removed '{}'.", target),
            Err(e) => format!("Failed to remove '{}': {}", target, e),
        }
    }


pub fn cargo_call(cwd: PathBuf, target: String, args: Vec<String>) -> String {
    let joined = cwd.join(&target);
    let target_path = match joined.canonicalize() {
        Ok(p) => p,
        Err(_) => return format!("Path '{}' does not exist!", target),
    };
    if !target_path.starts_with(&cwd) {
        return format!("Access denied: '{}' is outside the working directory.", target);
    }
    if !target_path.is_dir() {
        return format!("'{}' is not a directory.", target);
    }
    if !target_path.join("Cargo.toml").exists() {
        return format!("'{}' is not a Cargo project.", target);
    }

    let output = match Command::new("cargo")
        .args(&args)
        .current_dir(&target_path)
        .output()
    {
        Ok(o) => o,
        Err(e) => return format!("Failed to execute cargo: {}", e),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        if stdout.trim().is_empty() {
            format!("Cargo command completed successfully.")
        } else {
            stdout.into_owned()
        }
    } else {
        format!(
            "Cargo command failed (exit code {:?})\n\nstdout:\n{}\n\nstderr:\n{}",
            output.status.code(),
            stdout,
            stderr
        )
    }
}


    fn mode_to_string(mode: u32) -> String {
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



}