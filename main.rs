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

fn usage(filepath: &str) {
    eprintln!("Usage: {} [-n | --num] [-s | --sep]", filepath);
    eprintln!();
    eprintln!("Options");
    eprintln!("--num, -n    number of words");
    eprintln!("--sep, -s    word separator");
}

fn main() {
    let mut args = env::args();
    let filepath = args.next().expect("program filepath");
    let mut config = Config::default();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-n" | "--num" => {
                if let Some(next) = args.next() {
                    if let Ok(n) = next.parse::<usize>() {
                        config.num = n;
                    } else {
                        usage(filepath.as_str());
                        return;
                    }
                } else {
                    usage(filepath.as_str());
                    return;
                }
            }
            "-s" | "-sep" => {
                if let Some(next) = args.next() {
                    config.sep = next;
                } else {
                    usage(filepath.as_str());
                    return;
                }
            }
            _ => {
                usage(filepath.as_str());
                return;
            }
        }
    }

    let mut rng = rand::thread_rng();
    let mut words = WORDLIST.lines().collect::<Vec<_>>();
    words.shuffle(&mut rng);

    let passphrase = &words[0..config.num].join(config.sep.as_str());

    println!("{}", passphrase);
}
