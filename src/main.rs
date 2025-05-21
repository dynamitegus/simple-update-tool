use os_info;
use os_info::Type::Fedora;
use os_info::Type::Ubuntu;
use std::env;
use std::process::exit;

use std::process::Command;

/// Check the referenced vector for the passed arged_arg which is meant to be a single character string.
/// # Example
/// ```
/// if check_my_split_args(&args, "h") {
///     function();
/// }
/// ```
/// args being a goofy vector
fn check_my_split_args(args: &[&str], arged_arg: &str) -> bool {
    for arg in args {
        if arged_arg == *arg {
            return true;
        }
    }
    false // Move this outside the loop to check all arguments
}

/// ```
fn help() {
    println!("HELP");
    let output = Command::new("/bin/cat")
        .arg("file.txt")
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    assert!(output.status.success());
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
/// Check the passed vector
fn one_tack(args: Vec<&str>) {
    if check_my_split_args(&args, "h") {
        help();
    }
    if check_my_split_args(&args, "r") {
        update_repos();
    }
    if check_my_split_args(&args, "u") {
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
/// ```
fn main() {
    let info = os_info::get();

    let passed_args: Vec<String> = env::args().collect();

    if passed_args.len() < 2 {
        println!("No arguments provided. Use --help for usage information.");
        exit(1);
    }

    let split_args = &passed_args[1].split("");

    let my_args_have_been_splited = split_args.clone().collect::<Vec<&str>>();

    println!("{}", &passed_args[1]);

    match info.os_type() {
        Fedora => {
            // Continue with Fedora-specific logic
            if passed_args[1].starts_with("--") {
                two_tack(passed_args.clone());
            } else if passed_args[1].starts_with("-") {
                // Extract the characters after the dash

                let flags = passed_args[1]
                    .chars()
                    .skip(1)
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>();

                one_tack(flags.iter().map(|s| s.as_str()).collect());
            } else {
                println!("Command must start with - or --. Use --help for usage information.");
            }
        }
        _ => {
            println!("Operating System not supported yet! Go make a pull request and add support! Here is the repository link: https://github.com/dynamitegus/simple-update-tool/");

            exit(1);
        }
    }
}
