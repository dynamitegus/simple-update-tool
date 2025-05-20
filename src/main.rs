use std::env;

fn check_my_split_args(args: Vec<&str>, arged_arg: &str) -> bool {
    for arg in args {
        if arged_arg == arg {
            return true;
        }
    }
    false // Move this outside the loop to check all arguments
}

fn help() {
    println!("HELP");
}

fn invald_tack() {
    println!("Please enter a valid tack command or use --help for help.")
}

fn one_tack(args: Vec<&str>) {
    if check_my_split_args(args, "h") {
        help();
    } else {
        invald_tack();
    }
    
}

fn two_tack(args: Vec<String>) {
    if args.contains(&String::from("--help")) {
        help();
    } else {
        invald_tack();
    }
}

fn main() {
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
