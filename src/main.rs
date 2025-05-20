use std::env;
use std::process::exit;
use os_info;
use os_info::Type::Fedora;
use os_info::Type::Ubuntu;
/// ```
fn check_my_split_args(args: Vec<&str>, arged_arg: &str) -> bool {
    for arg in args {
        if arged_arg == arg {
            return true;
        }
    }
    false // Move this outside the loop to check all arguments
}

/// ```
fn help() {
    println!("HELP");
}
/// ```
fn update_repos() {
    println!("update repos")
}
/// ```
fn upgrade() {
    println!("upgrade")
}
/// ```
fn invald_tack() {
    println!("Please enter a valid tack command or use --help for help.")
}

/// ```
fn one_tack(args: Vec<&str>) {
    if check_my_split_args(args.clone(), "h") {
        help();
    } 
    if check_my_split_args(args.clone(), "r") {
        update_repos();
    }
    if check_my_split_args(args.clone(), "u") {
        upgrade();
    }
    
}

/// ```
fn two_tack(args: Vec<String>) {
    if args.contains(&String::from("--help")) {
        help();
    } else if args.contains(&String::from("--update")) {
        update_repos();
    } else if args.contains(&String::from("--upgrade")) {
        upgrade();
    } else {
        invald_tack();
    }
}

/// Entry point for the command-line tool, handling OS compatibility and argument parsing.
///
/// Checks if the operating system is Fedora and exits with an error message if not. Parses command-line arguments to determine if single-dash short flags or double-dash long flags are used, then dispatches to the appropriate handler function. Prints an error message if the argument format is unrecognized.
///
/// # Examples
///
/// ```
/// // Run the compiled binary with arguments, e.g.:
/// // $ ./simple-update-tool --help
/// // $ ./simple-update-tool -h
/// main();
/// ```
fn main() {
    let info = os_info::get();

    if info.os_type() != Fedora {
        println!("Operating System not supported yet! Go make a pull request and add support! Here is the repository link: https://github.com/dynamitegus/simple-update-tool/");
        
        exit(1);
    }

    let passed_args: Vec<String> = env::args().collect();
    let split_args = &passed_args[1].split("");
    let my_args_have_been_splited = split_args.clone().collect::<Vec<&str>>();
    println!("{}", &passed_args[1]);

    if (my_args_have_been_splited[1] == "-") && (my_args_have_been_splited[2] == "-") {
        two_tack(passed_args.clone());
    } else if (my_args_have_been_splited[1] == "-") && (my_args_have_been_splited[2] != "-") {
        one_tack(my_args_have_been_splited);
    } else {
        println!("UH OH NO TACK");
    }

}
