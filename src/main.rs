use std::process::Command;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("grith")
        .version("0.0.1")
        .author("linglx")
        .about("A git order improved cammand tool written in Rust")
        .subcommand(
            SubCommand::with_name("tag")
                .about("Display tag name with description.")
                .arg(
                    Arg::with_name("name")
                        .help("target name to display")
                        .takes_value(true)
                        .empty_values(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("branch")
                .about("Display branch name with description")
                .arg(
                    Arg::with_name("name")
                        .help("target name to display")
                        .takes_value(true)
                        .empty_values(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("branch", Some(sub_m)) => {
            display_doc_of_branch(sub_m.value_of("name"));
        }
        ("tag", Some(sub_m)) => {
            display_doc_of_tag(sub_m.value_of("name"));
        }
        (command, _) => unreachable!("Invalid subcommand: {}", command),
    }
}

fn display_doc_of_branch(branch_name: Option<&str>) {
    let command = "git";
    match branch_name {
        Some(_branch_name) => {
            let order = format!("branch.{}.description", _branch_name);
            let order = order.as_str();
            let orders = vec!["config", order];
            println!("the order is: {} {}", command, orders.join(" "));
            println!("{} --> {}", _branch_name, excute_command(command, orders));
        }
        None => {
            let branches = excute_command(command, vec!["branch", "--list"]);
            let branch_names = split_str_to_branches(branches.as_str());
            println!("{:?}", branch_names);
            for branch_name in branch_names.iter() {
                let order = format!("branch.{}.description", clean_branch_name(branch_name));
                let orders = vec!["config", order.as_str()];
                println!("{} --> {}", branch_name, excute_command(command, orders));
            }
        }
    }
}

fn clean_branch_name(branch_name: &str) -> &str {
    // 正则表达式清除无关字符
    return branch_name.trim_start_matches("*").trim();
}

fn split_str_to_branches(branches: &str) -> Vec<&str> {
    return branches.trim().split("\n").collect();
}

fn display_doc_of_tag(tag_name: Option<&str>) {
    let command = "git";
    let orders;
    match tag_name {
        None => {
            orders = vec!["tag", "-l", "-n"];
        }
        Some(_tag_name) => {
            orders = vec!["tag", "-l", "-n", _tag_name];
        }
    };

    println!("the order is: {} {}", command, orders.join(" "));
    println!("{}", excute_command(command, orders));
}

fn excute_command(command: &str, args: Vec<&str>) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("happen error.");
    return if output.status.success() {
        format!("{}", String::from_utf8_lossy(&output.stdout).trim())
    } else {
        format!("{}", String::from_utf8_lossy(&output.stderr))
    };
}

