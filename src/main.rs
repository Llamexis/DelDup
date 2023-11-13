mod config;
use config::*;
use sha2::Digest;
use sha2::Sha512;
use std::io;
use std::path::Path;
use std::process::exit;
use std::{collections::HashMap, env, fs, io::prelude::*, path::PathBuf};

fn travel_dirs(conf: &Config) -> Result<(Vec<PathBuf>, Vec<PathBuf>), std::io::Error> {
    let mut files = vec![];
    let mut dirs = vec![];

    let mut open = conf.paths();

    while let Some(path) = open.pop() {
        for content in fs::read_dir(path)? {
            let content = content?;
            if conf.is_verbose() {
                println!("FOUND: {:?}", content.path());
            }
            match content.path() {
                x if x.is_dir() => {
                    if conf.is_recursive() {
                        open.push(x.clone());
                    }
                    dirs.push(x);
                }
                x if x.is_file() => files.push(x),
                _ => {}
            }
        }
    }

    Ok((dirs, files))
}

fn save_to_file(
    hashmap: &HashMap<String, Vec<PathBuf>>,
    path: String,
) -> Result<(), std::io::Error> {
    let mut path = path;
    if !path.ends_with(".json") {
        path.push_str(".json");
    }

    let mut file = fs::File::create(path)?;
    writeln!(file, "[")?;

    for (idx, (hash, files)) in hashmap.iter().enumerate() {
        if idx == hashmap.len() - 1 {
            writeln!(file, "\t\"{}\" : {:?}", hash, files)?;
        } else {
            writeln!(file, "\t\"{}\" : {:?},", hash, files)?;
        }
    }

    writeln!(file, "]")?;
    Ok(())
}

fn translate_to_hashmap(
    files: Vec<PathBuf>,
    conf: &Config,
) -> Result<HashMap<String, Vec<PathBuf>>, std::io::Error> {
    let mut hashmap = HashMap::new();
    for file_path in files {
        let mut sha = Sha512::new();
        let mut file = fs::read(&file_path)?;

        sha.update(&mut file);
        let hash: String = sha
            .finalize()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<Vec<_>>()
            .join("");
        hashmap
            .entry(hash.clone())
            .or_insert(Vec::new())
            .push(file_path.clone());
        if conf.is_verbose() && hashmap.get(&hash).unwrap().len() > 1 {
            println!("FOUND DUPLICATE: {file_path:?}");
        }
    }

    Ok(hashmap)
}

fn delete_duplicates(hashmap: &HashMap<String, Vec<PathBuf>>) {
    let mut iter = 0u128;
    for file_paths in hashmap.values() {
        if file_paths.len() == 1 {
            continue;
        }
        for file in file_paths.iter().skip(1) {
            match fs::remove_file(file) {
                Err(err) => eprintln!("{:?}", err),
                Ok(_) => println!(
                    "{iter}: \"{}\" -> \"{}\"",
                    file_paths[0].to_str().unwrap(),
                    file.to_str().unwrap()
                ),
            };
        }
        iter += 1;
    }
}

fn main() -> Result<(), std::io::Error> {
    let conf = parse_args();
    let (_, files) = travel_dirs(&conf)?;
    let hashmap = translate_to_hashmap(files, &conf)?;

    if conf.is_output() {
        save_to_file(&hashmap, "output.json".to_owned())?;
    } else {
        delete_duplicates(&hashmap);
    }
    Ok(())
}
