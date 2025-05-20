use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tack_args = &args[1];
    println!("{}", &args[1]);
    if args[1] == "-h" {
        println!("HELP")
    };

}
