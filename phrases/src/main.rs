extern crate phrases;

use phrases::english;
use phrases::japanese;

fn main() {
    println!("Hello in English: {}", english::hello());
    println!("Goodbye in English: {}", english::goodbye());

    println!("Hello in Japanese: {}", japanese::hello());
    println!("Goodbye in Japanese: {}", japanese::goodbye());
}
