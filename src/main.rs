extern crate lapp;

fn main() {
    let args = lapp::parse_args("
    Prints out first n lines of a file
        <makefile> (default 'make.toml') name of makefile
        <mode> (default 'dev') compilation mode
    ");

    let makefile_name = args.get_string("makefile");
    let compilation_mode = args.get_string("mode");

    println!("makefile: '{}', compilation mode: '{}'",
        makefile_name, compilation_mode);
}
