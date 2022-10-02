//
pub fn return_cli_arguments<'a>(args: &'a mut Vec<String>) ->  &'a str {

    // Only one argument allowed
    if args.len() > 2 { panic!("No second argument allowed") }


    // return a &str. With our lifetimes, we can guarantee that the &str will live as long as the
    // `args` argument prompted into the function
    &args[1]
}
