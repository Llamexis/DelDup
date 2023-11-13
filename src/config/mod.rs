use std::{path::PathBuf, env, process::exit, fs};

pub struct Config {
    is_recursive: bool,
    is_verbose: bool,
    is_output: bool,
    paths: Vec<PathBuf>,
}
impl Config {
    pub fn is_recursive(&self) -> bool {
        self.is_recursive 
    }
    pub fn is_verbose(&self) -> bool {
        self.is_recursive
    }
    pub fn is_output(&self) -> bool {
        self.is_output
    }
    pub fn paths(&self) -> Vec<PathBuf> {
        self.paths.clone()
    }
}

pub fn print_usage() {
    let msg = format!(
        "Usage: {} [options] target ...
    \toptions:
    \t\t-r,\t\trecursive
    \t\t-o,\t\toutput program will output duplicates in output.json
    \t\t-v,\t\tverbose",
        env::args().collect::<Vec<_>>()[0]
    );
    println!("{}", msg);
}

pub fn parse_args() -> Config {
    let mut conf = Config {
        is_recursive: false,
        is_verbose: false,
        is_output: false,
        paths: vec![],
    };
    let mut args = env::args();
    args.next();

    for arg in args {
        if arg.starts_with("-") {
            for ch in arg.chars() {
                match ch {
                    'v' => conf.is_verbose = true,
                    'r' => conf.is_recursive = true,
                    'o' => conf.is_output = true,
                    '-' => (),
                    _ => {
                        print_usage();
                        exit(1);
                    }
                }
            }
        } else {
            let path = match fs::canonicalize(&arg) {
                Ok(p) if p.is_dir() => p,
                Ok(_p) => {
                    print_usage();
                    exit(1);
                }
                Err(_e) => {
                    print_usage();
                    exit(1);
                }
            };
            conf.paths.push(path);
        }
    }
    if conf.paths.is_empty() {
        print_usage();
        exit(1);
    }

    conf
}
