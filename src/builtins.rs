use std::env;

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
