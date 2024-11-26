//extern crate ferris_says;

//use ferris_says::say;
use std::io::{stdout, BufWriter, self};

fn main() {
    let mut count = 0;
        while count == 0 {  
            let out = "Hello fellow CoCo !";
            let width = 12;   
            let mut writer = BufWriter::new(stdout());
            ferris_says::say(out, width, &mut writer).unwrap();
            count = 1;
        }
    println!("\nPRESS ENTER");
    let _ = io::stdin().read_line(&mut String::new());
}