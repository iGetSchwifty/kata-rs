use std::env;
use kata;

fn main() {
    match kata::run(env::args().collect()) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        },
        Err(error) => {
            println!("{}", error);
        }
    }   
}
