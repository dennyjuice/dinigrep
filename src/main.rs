use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Ищем {}\nв файле {}", query, filename);

    let contents = fs::read_to_string(filename).expect("Шо-то пошло на хуй");

    println!("с тектом: {}", contents);
}
