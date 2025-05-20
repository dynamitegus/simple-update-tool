use std::env;

fn check_my_split_args(args: Vec<&str>, arged_arg: &str) -> bool {
    for arg in args {
        if arged_arg == arg {
            return true;
        }
    }
    false // Move this outside the loop to check all arguments
}

fn main() {
    let passed_args: Vec<String> = env::args().collect();
    let split_args = &passed_args[1].split("");
    let my_args_have_been_splited = split_args.clone().collect::<Vec<&str>>();
    println!("{}", &passed_args[1]);

    if (my_args_have_been_splited[1] == "-") && (my_args_have_been_splited[2] == "-") {
        println!("two tack");
    } else if (my_args_have_been_splited[1] == "-") && (my_args_have_been_splited[2] != "-") {
        println!("one tack");
    } else {
        println!("UH OH NO TACK");
    }

    if check_my_split_args(my_args_have_been_splited, "h") {
        println!("HELP");
    };
}
