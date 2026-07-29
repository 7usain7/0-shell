use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
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

pub fn cp(args: &[String]) {
    if args.len() < 2 {
        eprintln!("cp: missing file operand");
        return;
    }
    let dest_str = &args[args.len() - 1];
    let dest_path = Path::new(dest_str);
    let is_dir = dest_path.is_dir();

    if args.len() > 2 && !is_dir {
        eprintln!("cp: target '{}' is not a directory", dest_str);
        return;
    }

    for src_str in &args[..args.len() - 1] {
        let src_path = Path::new(src_str);
        let mut final_dest = dest_path.to_path_buf();
        if is_dir {
            if let Some(file_name) = src_path.file_name() {
                final_dest.push(file_name);
            }
        }
        if let Err(e) = fs::copy(src_path, &final_dest) {
            eprintln!("cp: cannot copy '{}' to '{}': {}", src_str, final_dest.display(), e);
        }
    }
}

pub fn mv(args: &[String]) {
    if args.len() < 2 {
        eprintln!("mv: missing file operand");
        return;
    }
    let dest_str = &args[args.len() - 1];
    let dest_path = Path::new(dest_str);
    let is_dir = dest_path.is_dir();

    if args.len() > 2 && !is_dir {
        eprintln!("mv: target '{}' is not a directory", dest_str);
        return;
    }

    for src_str in &args[..args.len() - 1] {
        let src_path = Path::new(src_str);
        let mut final_dest = dest_path.to_path_buf();
        if is_dir {
            if let Some(file_name) = src_path.file_name() {
                final_dest.push(file_name);
            }
        }
        if let Err(e) = fs::rename(src_path, &final_dest) {
            eprintln!("mv: cannot move '{}' to '{}': {}", src_str, final_dest.display(), e);
        }
    }
}

pub fn ls(args: &[String]) {
    let mut show_all = false;
    let mut show_indicators = false;
    let mut paths = Vec::new();

    for arg in args {
        if arg.starts_with('-') && arg.len() > 1 {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => show_all = true,
                    'F' => show_indicators = true,
                    _ => {}
                }
            }
        } else {
            paths.push(arg.clone());
        }
    }

    if paths.is_empty() {
        paths.push(String::from("."));
    }

    for (i, path_str) in paths.iter().enumerate() {
        if paths.len() > 1 {
            if i > 0 {
                println!();
            }
            println!("{}:", path_str);
        }

        let path = Path::new(path_str);
        if !path.exists() {
            eprintln!("ls: cannot access '{}': No such file or directory", path_str);
            continue;
        }

        if path.is_dir() {
            if let Ok(read_dir) = fs::read_dir(path) {
                let mut entries = Vec::new();
                for entry in read_dir.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if show_all || !name.starts_with('.') {
                        let mut display_name = name;
                        if show_indicators {
                            display_name.push_str(&get_indicator(&entry.path()));
                        }
                        entries.push(display_name);
                    }
                }
                entries.sort();
                println!("{}", entries.join("  "));
            }
        } else {
            let mut display_name = path_str.to_string();
            if show_indicators {
                display_name.push_str(&get_indicator(path));
            }
            println!("{}", display_name);
        }
    }
}

fn get_indicator(path: &Path) -> String {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_symlink() {
            return String::from("@");
        } else if meta.is_dir() {
            return String::from("/");
        } else if meta.permissions().mode() & 0o111 != 0 {
            return String::from("*");
        }
    }
    String::new()
}
