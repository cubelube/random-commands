use std::env;
use std::fs;
use std::io::Read;
use colored::Colorize;

pub fn perform()
{
    match env::args().nth(1).expect("failed").as_str()
    {
        "read" =>
        {
            if env::args().nth(3).expect("failed") == "true"
            {
                print_lines(env::args().nth(2).expect("failed").as_str(), true);

            }
            else
            {
                print_lines(env::args().nth(2).expect("failed").as_str(), false);

            }
        },

        "find" =>
        {
            let third_arg = env::args().nth(3).expect("failed");
            if env::args().nth(4).expect("failed") == "true"
            {
                find_in_file(&third_arg, &env::args().nth(2).expect("failed"), true);
            }
            else
            {
                find_in_file(&third_arg, &env::args().nth(2).expect("failed"), false);
            }
        },

        "print" => println!("{}", echo(env::args().nth(2).expect("failed"))),

        "bin" => println!("{}", bin_to_dec(env::args().nth(2).expect("failed"))),

        "dec" => println!("{}", dec_to_bin(env::args().nth(2).expect("failed").parse().expect("failed"))),

        _ =>
        {
            println!("\n{}", "SYNTAX".bright_cyan().bold().underline());
            println!("\n{} {} {}", "READ FILE:".yellow(), "cargo run -- read".bold(), "\"path_to_file\"".green().bold());
            println!("{} {} {}", "FIND IN FILE:".yellow(), "cargo run -- find".bold(), "\"search_string\" \"path_to_file\"".green().bold());
            println!("{}", "^ TOGGLEABLE LINE COUNTER: Append \"true\" or \"false\" to the end of command to toggle line counters\n".blue().bold());
            println!("{} {} {}", "ECHO:".yellow(), "cargo run -- print".bold(), "\"string\"".green().bold());
            println!("{} {} {}", "BINARY TO DECIMAL:".yellow(), "cargo run -- bin".bold(), "\"binary_number\"".green().bold());
            println!("{} {} {}", "DECIMAL TO BINARY:".yellow(), "cargo run -- dec".bold(), "\"decimal_number\"".green().bold());
        }
    };
}

fn print_lines(input: &str, have_line_counter: bool)
{
    let mut content = String::new();

    let mut f = fs::OpenOptions::new().write(true).create(true).read(true).open(input).expect("failed");

    let _ = f.read_to_string(&mut content);

    if have_line_counter
    {
        let mut line_counter = 1;
        for line in content.lines()
        {
            println!("{}. {}", line_counter, line);
            line_counter += 1;
        }
    }
    else
    {
        for line in content.lines()
        {
            println!("{}", line);
        }
    }
}

fn find_in_file(file_name: &str, search_str: &str, have_line_counter: bool)
{
    let mut content = String::new();

    let mut f = fs::OpenOptions::new().write(true).create(true).read(true).open(file_name).expect("failed");

    let _ = f.read_to_string(&mut content);

    if have_line_counter
    {
        let mut line_counter = 1;

        for line in content.lines()
        {
            if line.find(search_str).is_some()
            {
                println!("{}. {}", line_counter, line);
            }
            line_counter += 1;
        }
    }
    else
    {
        for line in content.lines()
        {
            if line.find(search_str).is_some()
            {
                println!("{}", line);
            }
        }
    }
}

fn echo(input: String) -> String
{
    String::from(input)
}

fn bin_to_dec(mut input: i32) -> i32
{
    let mut exp = 0;
    let mut sum: i32 = 0;

    while input > 0
    {
        if input % 10 != 0
        {
            sum += i32::pow(2, exp);
        }

        exp += 1;
        input /= 10
    }

    sum
}

fn dec_to_bin(mut input: i32) -> String
{
    let mut sum = String::new();

    while input > 0
    {
        sum.push_str(&(input % 2).to_string());

        input /= 2;
    }

    sum.chars().rev().collect::<String>()
}
