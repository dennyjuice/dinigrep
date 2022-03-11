use std::env;
use std::process;

use dinigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Проблемка: {}", err);
        process::exit(1);
    });

    println!("Ищем {}\nВ файле {}", config.query, config.filename);

    if let Err(e) = dinigrep::run(config) {
        println!("Шо-то пошло на хуй таким способом: {}", e);

        process::exit(1);
    };
}
