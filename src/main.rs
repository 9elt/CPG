mod chars;
mod log;
mod salt;

use chars::CHAR_TABLE;
use clap::Parser;

use bcrypt_pbkdf::bcrypt_pbkdf;

use time_elapsed;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// passwords to salt
    #[arg(short, long, num_args = 0..)]
    passowrds: Vec<String>,

    /// salt path (optional)
    #[arg(short, long)]
    salt: Option<String>,
}

fn main() {
    let args = Args::parse();

    let salt = salt::from_path(args.salt);

    if args.passowrds.len() == 0 {
        return ();
    }

    if args.passowrds[0].as_str() == "benchmark" {
        return benchmark();
    }

    log::line();

    if let Ok(salt) = salt {
        for psw in &args.passowrds {
            log::password(&psw, &strengthen(&psw, &salt));
        }
    }

    log::line();
}

fn benchmark() {
    let salt = salt::random_salt();

    let passwords: Vec<String> = (0..u8::MAX)
        .into_iter()
        .map(|n| format!("{:012x}", n))
        .collect();

    let time = time_elapsed::start("benchmark");

    for psw in &passwords {
        strengthen(&psw, &salt);
    }

    time.end();
}

fn strengthen(psw: &String, salt: &Vec<u8>) -> String {
    let mut hash = vec![0; 32];

    match bcrypt_pbkdf(psw, salt, 32, &mut hash) {
        Ok(_) => (),
        Err(err) => println!("error {:?}", err),
    };

    hash.iter()
        .map(|byte| CHAR_TABLE[*byte as usize])
        .collect::<String>()
}
