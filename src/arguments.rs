// exit function, Used to disregard panic!() messages
use std::process::exit;

pub fn return_cli_arguments(cli_args: Vec<String>) -> Flags {

    // Only one argument allowed
    if cli_args.len() < 3 { 
        panic!("Not enough arguments prompted!");
    }

    // Instantiate a Flags struct
    let mut _flag = Flags::new();

    // Loop over the `args` argument, which are cli_arguments
    for (i, _arg) in cli_args.iter().enumerate() {
        if _arg == "--peptide" { 
            _flag.add_torsion_and_interval_fields(&i, _arg, &cli_args)
        };
        if _arg == "--fivering" { 
            _flag.add_torsion_and_interval_fields(&i, _arg, &cli_args)
        };
        if _arg == "--sixring" { 
            _flag.add_torsion_and_interval_fields(&i, _arg, &cli_args)
        };

        if _arg == "--twopi" { _flag.twopi = true };
        if _arg == "--rad" { _flag.rad = true };

    }
    // println!("{:?}", _flag);
    // return the Flags struct with all the needed values
    _flag
}


// Define Flags struct
#[derive(Debug)]
pub struct Flags {
    pub torsion_type : String,
    pub rad : bool,
    pub interval : usize,
    pub twopi : bool 
}

// Implemented amount of methods : 2
impl Flags {
    fn new() -> Flags{ //torsions : String, intv : usize
        Flags {
            torsion_type : "".to_owned(), // converts &str type to String type
            rad : false,
            twopi : false,
            interval : 0
        }
    
    }

    fn add_torsion_and_interval_fields(&mut self, idx : &usize, _arg : &String, args : &Vec<String>) {
        // take ownership of the argument value and take it
        self.torsion_type = _arg.to_owned();

        // Retrieve the next value in the Vec<> and check if it is parsable as a `usize`
        self.interval = match args[idx + 1].parse::<usize>() {
            Ok(integer) => integer,
            Err(_) => interval_not_prompted_correctly()
        }

    }
}



#[allow(unreachable_code)]
fn interval_not_prompted_correctly() -> usize {
    println!("Prompted INTERVAL is not an integer value or was not prompted at all.");
    exit(1);
    0 // Hacky shit to make the matching arm not complain

}
