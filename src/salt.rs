use super::log;
use home;
use rand::Rng;
use std::ffi::OsStr;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::{fs, fs::File};

/// ## random salt generation
/// 
/// creates a new random salt
/// 
pub fn random_salt() -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let mut salt: Vec<u8> = Vec::with_capacity(256);

    for _ in 0..256 {
        salt.push(rng.gen())
    }

    salt
}

const DEFAULT_SALT_PATH: &str = ".config/cpg/secret.salt";

pub fn from_path(path: Option<String>) -> Result<Vec<u8>, ()> {
    let path = match path {
        Some(path) => Path::new(&path).to_path_buf(),
        None => match home::home_dir() {
            Some(mut home) => {
                home.push(DEFAULT_SALT_PATH);
                home
            }
            None => {
                log::missing_home();
                return Err(());
            }
        },
    };

    match path.exists() {
        true => load_from_path(&path),
        false => init_from_path(&path),
    }
}

fn load_from_path(path: &PathBuf) -> Result<Vec<u8>, ()> {
    let mut bytes: Vec<u8> = Vec::with_capacity(2048);

    match File::open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);

            match reader.read_to_end(&mut bytes) {
                Ok(_) => (),
                Err(_) => {
                    log::cant_read_file(path);
                    return Err(());
                }
            }
        }
        Err(_) => {
            log::cant_open_file(path);
            return Err(());
        }
    };

    Ok(bytes)
}

fn init_from_path(path: &PathBuf) -> Result<Vec<u8>, ()> {
    log::path_not_found(path);

    match path.extension() {
        Some(extension) => {
            if extension != OsStr::new("salt") {
                log::wrong_extension();
                return Err(());
            }
        }
        None => {
            log::missing_filename();
            return Err(());
        }
    }

    if !log::init_confirmation() {
        return Err(());
    }

    if let Some(dir) = path.parent() {
        if !dir.exists() {
            if let Err(_) = fs::create_dir_all(dir) {
                log::cant_create_dir(path);
                return Err(());
            };
        }
    }

    let salt = random_salt();

    match fs::write(path, salt.to_owned()) {
        Ok(_) => {
            log::salt_created(path);
            log::suggestion();
        }
        Err(_) => {
            log::cant_create_salt_file(path);
            return Err(());
        }
    };

    Ok(salt)
}
