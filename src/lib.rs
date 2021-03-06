mod log;
mod seek;
mod storage;

use anyhow::Result;
use std::io;
use std::path::{Path, PathBuf};
use storage::KVStorage;

const SET: &str = "set";
const RM: &str = "rm";
const GET: &str = "get";
const EXIT: &str = "exit";

pub fn run<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut input = String::new();
    let mut storage: KVStorage = KVStorage::new(path)?;
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read command");
        input = input.trim().to_string();
        let args: Vec<&str> = input.trim().split(' ').collect();
        match args[0] {
            SET => storage.insert(args[1].to_string(), args[2].to_string())?,
            RM => {
                if let Err(x) = storage.delete(args[1].to_string()) {
                    println!("{}", x)
                }
            }
            GET => match storage.get(args[1].to_string()) {
                Ok(Some(x)) => println!("{}", x),
                Ok(None) => eprintln!("Key wasn't found"),
                Err(e) => eprintln!("{}", e),
            },
            EXIT => break,
            x => eprintln!("Unknown command {}", x),
        }
        input.clear();
    }
    Ok(())
}
