use std::{
    env,
    fs::File,
    path::PathBuf
};




fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let xml = &args[0];
    let ron = &args[1];
    println!("Target .xml: {}", xml);
    println!("Target .ron: {}", ron);
}
