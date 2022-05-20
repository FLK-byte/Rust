use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main(){
    let stdout = stdout();
    let message= String::from("Salve ferris!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    
    println!("E esse também");
    say(message.as_bytes(), width, &mut writer).unwrap();
    println!("Porque esse output vem antes do say()")
    
}