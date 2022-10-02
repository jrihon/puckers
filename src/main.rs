// Standard libs
use std::env::args;



// Custom libs
mod arguments;


// MAIN
fn main() {

    let mut _args: Vec<String> = args().collect();

    match arguments::return_cli_arguments(&mut _args) {
        "pucker" =>  equidistant_globe(),
        _ => panic!("Flag Not Found")
    };
}



fn equidistant_globe() { println!("equidistant globe") }
