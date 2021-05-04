use clap::{App, Arg};
use rand::rngs::OsRng;
use rand::RngCore;
use std::process;

fn main() {
    let matches = App::new("Password Generator")
        .author("MFRIbrahim")
        .about(
"This tool creates a random password of given length. All 26 english letters are used in their lower
and upper case form as well as the numbers 0 to 9. In addition to that the the user can provide an
optional argument that contains all additional characters the user wants to be included."
        )
        .arg(
            Arg::with_name("length")
                .help(
"The desired length of the password. "
                )
                .required(true)
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("characters")
                .takes_value(true)
                .default_value("")
        )
        .get_matches();

    let password_length = match matches.value_of("length").unwrap().parse::<usize>() {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Given length is not a positive integer.");
            process::exit(1);
        }
    };
    let mut additional_chars = matches
        .value_of("config")
        .unwrap()
        .to_string()
        .chars()
        .collect();

    let mut valid_chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                  abcdefghijklmnopqrstuvwxyz\
                                  0123456789"
        .chars()
        .collect();
    valid_chars.append(&mut additional_chars);

    let password = get_random_password(&valid_chars, &password_length);
    println!("random password: {}", password);
}

fn get_random_password(valid_chars: &Vec<char>, password_length: &usize) -> String {
    let mut password = String::new();
    for _ in 0..*password_length {
        let random_u64 = OsRng.next_u64();
        password.push(valid_chars[(random_u64 as usize) % valid_chars.len()]);
    }

    password
}
