use rand::prelude::*;
use std::env;

struct Config {
    num: usize,
    sep: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            num: 4,
            sep: "-".to_string(),
        }
    }
}

static WORDLIST: &'static str = include_str!("eff_short_wordlist_1.txt");

fn usage() {
    let filepath = env::args().next().unwrap();
    eprintln!("Usage: {} [-n | --num] [-s | --sep]", filepath);
    eprintln!();
    eprintln!("Options");
    eprintln!("--num, -n    number of words");
    eprintln!("--sep, -s    word separator");
}

fn parse_args() -> Result<Config, ()> {
    let mut args = env::args();
    args.next();
    let mut config = Config::default();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-n" | "--num" => {
                if let Some(next) = args.next() {
                    if let Ok(n) = next.parse::<usize>() {
                        config.num = n;
                    } else {
                        return Err(());
                    }
                } else {
                    return Err(());
                }
            }
            "-s" | "--sep" => {
                if let Some(next) = args.next() {
                    config.sep = next;
                } else {
                    return Err(());
                }
            }
            _ => {
                return Err(());
            }
        }
    }

    Ok(config)
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut words = WORDLIST.lines().collect::<Vec<_>>();
    words.shuffle(&mut rng);
    let config = match parse_args() {
        Ok(cfg) => cfg,
        _ => {
            usage();
            return;
        }
    };

    let passphrase = &words[0..config.num].join(config.sep.as_str());

    println!("{}", passphrase);
}
