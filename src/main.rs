use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::io;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow CoCo !");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width , &mut writer).unwrap();
    //fsays --width 12 ("Hello fellow Rustaceans!");

    println!("{}", "\nPress ENTER".to_string());
    let _ = io::stdin().read_line(&mut String::new());
}