//
pub fn return_cli_arguments<'a>(args: &'a mut Vec<String>) ->  &'a str {

    // Only one argument allowed
    if args.len() > 2 { panic!("No second argument allowed") }


    // return a &str. With our lifetimes, we can guarantee that the &str will live as long as the
    // `args` argument prompted into the function
    &args[1]
}


struct Flags {
    torsion_type : String,
    rad : bool,
    interval : usize,
    twopi : bool 
}

impl Flags {
    fn new(torsions : String, intv : usize) -> Flags{
        Flags {
            torsion_type : torsions,
            rad : false,
            twopi : false,
            interval : intv
        }
    
    }
}
