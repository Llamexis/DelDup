use sha2::Digest;
use sha2::Sha256;
use std::{collections::HashMap, env, fs, io::prelude::*, path::PathBuf};

#[allow(dead_code)]
fn travel_dirs(path: &String) -> Result<(Vec<PathBuf>, Vec<PathBuf>), std::io::Error> {
    let mut files = vec![];
    let mut dirs = vec![];
    let path = fs::canonicalize(path)?;

    for i in fs::read_dir(path)? {
        let i = i?;
        match i.path() {
            x if x.is_dir() => {
                dirs.push(x);
            }
            x if x.is_file() => {
                files.push(x);
            }
            _ => {}
        }
    }

    Ok((dirs, files))
}

fn travel_dirs_recursive(path: &str) -> Result<(Vec<PathBuf>, Vec<PathBuf>), std::io::Error> {
    let mut files = vec![];
    let mut dirs = vec![];
    let path = fs::canonicalize(path)?;

    for i in fs::read_dir(path)? {
        let i = i?;
        match i.path() {
            x if x.is_dir() => {
                dirs.push(x);
                let (mut d, mut f) = travel_dirs_recursive(i.path().to_str().unwrap())?;
                dirs.append(&mut d);
                files.append(&mut f);
            }
            x if x.is_file() => {
                files.push(x);
            }
            _ => {}
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
) -> Result<HashMap<String, Vec<PathBuf>>, std::io::Error> {
    let mut hashmap = HashMap::new();
    for file_path in files {
        let mut sha = Sha256::new();
        let mut file = fs::read(&file_path)?;

        sha.update(&mut file);
        let hash: String = sha
            .finalize()
            .iter()
            .map(|x| format!("{:02x}", x))
            .collect::<Vec<_>>()
            .join("");
        hashmap
            .entry(hash)
            .or_insert(Vec::new())
            .push(file_path.clone());
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
    let args: Vec<_> = env::args().collect();
    let (_, files) = travel_dirs_recursive(&args[1])?;

    let hashmap = translate_to_hashmap(files)?;

    delete_duplicates(&hashmap);

    save_to_file(&hashmap, "output.json".to_string())?;

    Ok(())
}
