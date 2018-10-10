mod request;
mod error;

use request::request;

fn main() {
    match request() {
        Ok(text) => println!("{}", text),
        Err(err) => println!("{}", err),
    }
}
