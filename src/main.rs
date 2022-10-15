/// Make a flag to allow the user to define range from -pi to pi || 0 to 2pi
///     like --2pi to explicitly define this and default to the other one
///
/// Make a flag to allow the returned arrays to be printed in radians
///     like --rad , this program defaults with printing to degrees
///
// Standard libs
use std::env::args;


// Custom libs
mod arguments;
mod peptipde;


// MAIN
fn main() {

    let mut _args: Vec<String> = args().collect();

    match arguments::return_cli_arguments(&mut _args) {
        "--peptide" =>  peptipde::peptide(),
        "--fivering" =>  equidistant_globe(),
        "--sixring" =>  equidistant_globe(),
        _ => panic!("Flag Not Found")
    };
}



fn equidistant_globe() { println!("equidistant globe") }
