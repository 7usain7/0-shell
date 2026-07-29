use std::env;
use std::fs;
use std::path::Path;

pub fn echo(args: &[String]) {
    println!("{}", args.join(" "));
}

pub fn pwd() {
    if let Ok(dir) = env::current_dir() {
        println!("{}", dir.display());
    }
}

pub fn cd(args: &[String]) {
    let path = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| String::from("/"))
    } else {
        args[0].clone()
    };

    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("cd: {}: {}", path, e);
    }
}

pub fn cat(args: &[String]) {
    for arg in args {
        match fs::read_to_string(arg) {
            Ok(contents) => print!("{}", contents),
            Err(e) => eprintln!("cat: {}: {}", arg, e),
        }
    }
}

pub fn mkdir(args: &[String]) {
    for arg in args {
        if let Err(e) = fs::create_dir_all(arg) {
            eprintln!("mkdir: {}: {}", arg, e);
        }
    }
}

pub fn touch(args: &[String]) {
    if args.is_empty() {
        eprintln!("touch: missing file operand");
        return;
    }
    for arg in args {
        if let Err(e) = fs::OpenOptions::new().write(true).create(true).open(arg) {
            eprintln!("touch: cannot touch '{}': {}", arg, e);
        }
    }
}

pub fn rm(args: &[String]) {
    let mut recursive = false;
    let mut targets = Vec::new();

    for arg in args {
        if arg == "-r" || arg == "-R" {
            recursive = true;
        } else {
            targets.push(arg);
        }
    }

    for target in targets {
        let path = Path::new(target);
        if path.is_dir() {
            if recursive {
                if let Err(e) = fs::remove_dir_all(path) {
                    eprintln!("rm: cannot remove '{}': {}", target, e);
                }
            } else {
                eprintln!("rm: cannot remove '{}': Is a directory", target);
            }
        } else {
            if let Err(e) = fs::remove_file(path) {
                eprintln!("rm: cannot remove '{}': {}", target, e);
            }
        }
    }
}
