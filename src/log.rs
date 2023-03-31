use std::path::PathBuf;
use std::io::{stdin, stdout, Write};

/// input => output
pub fn password(input: &String, output: &String) {
    println!("{} \x1b[90;1m=>\x1b[0m \x1b[1m{}\x1b[0m", input, output)
}

/// \n
pub fn line() {
    println!("")
}

/// cannot find home directory
pub fn missing_home() {
    line();
    println!("\x1b[31;1mcannot find home directory\x1b[0m")
}

/// cannot open salt file at: *path*
pub fn cant_open_file(path: &PathBuf) {
    line();
    println!("\x1b[31;1mcannot open salt file at:\x1b[0m {}", path.to_string_lossy())
}

/// cannot read salt file at: *path*
pub fn cant_read_file(path: &PathBuf) {
    line();
    println!("\x1b[31;1mcannot read salt file at:\x1b[0m {}", path.to_string_lossy())
}

/// salt not found at: *path*
pub fn path_not_found(path: &PathBuf) {
    line();
    println!("\x1b[31;1msalt not found at:\x1b[0m {}", path.to_string_lossy())
}

/// file should be a '.salt'
pub fn wrong_extension() {
    line();
    println!("\x1b[31;1mfile should be a '.salt'\x1b[0m")
}

/// missing filename
pub fn missing_filename() {
    line();
    println!("\x1b[31;1mmissing filename\x1b[0m")
}

/// cannot create directory at: *path*
pub fn cant_create_dir(path: &PathBuf) {
    line();
    println!("\x1b[31;1mcannot create directory at:\x1b[0m {}", path.to_string_lossy())
}

/// cannot create salt at: *path*
pub fn cant_create_salt_file(path: &PathBuf) {
    line();
    println!("\x1b[31;1mcannot create salt at:\x1b[0m {}", path.to_string_lossy())
}

/// salt created successfully at: *path*
pub fn salt_created(path: &PathBuf) {
    line();
    println!("\x1b[32;1msalt created successfully at:\x1b[0m {}", path.to_string_lossy())
}

/// please back it up in a SECURE place (possibly offline)
pub fn suggestion() {
    println!("\x1b[1mplease back it up in a SECURE place (possibly offline)\x1b[0m")
}

/// do you want to initialize a new one? [Y/n] 
pub fn init_confirmation() -> bool {
    print!("do you want to \x1b[1minitialize a new one\x1b[0m? [Y/n] ");
    confirmation()
}

/// request user confirmation (default Yes)
fn confirmation() -> bool {
    match stdout().flush() {
        Ok(_) => (),
        Err(_) => return false,
    };

    let mut res = String::new();

    match stdin().read_line(&mut res) {
        Ok(_) => (),
        Err(_) => return false,
    };

    match res.trim().to_ascii_uppercase().as_str() {
        "N" => false,
        _ => true,
    }
}
