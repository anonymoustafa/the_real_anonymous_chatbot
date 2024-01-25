use std::env;

fn main(){
    let args: Vec<String>=
    env::args().collect();

    println!("Program {}",args[0]);
    println!("Args... {:?}",&args[1..]);
}