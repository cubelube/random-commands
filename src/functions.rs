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

        "bin" => println!("{}", bin_to_dec(env::args().nth(2).expect("failed").parse().expect("failed"))),

        "dec" => println!("{}", dec_to_bin(env::args().nth(2).expect("failed").parse().expect("failed"))),

        "hex" => println!("{}", hex_to_dec(env::args().nth(2).expect("failed"))),

        "hdec" => println!("{}", dec_to_hex(env::args().nth(2).expect("failed").parse().expect("failed"))),

        _ =>
        {
            println!("\n{}", "SYNTAX".bright_cyan().bold().underline());
            println!("\n{} {} {}", "READ FILE:".yellow(), "cargo run -- read".bold(), r#""path_to_file""#.green().bold());
            println!("{} {} {}", "FIND IN FILE:".yellow(), "cargo run -- find".bold(), r#""search_string" "path_to_file""#.green().bold());
            println!("{}", r#"^ TOGGLEABLE LINE COUNTER: Append "true" or "false" to the end of command to toggle line counters\n"#.blue().bold());
            println!("{} {} {}", "ECHO:".yellow(), "cargo run -- print".bold(), r#""string""#.green().bold());
            println!("{} {} {}", "BINARY TO DECIMAL:".yellow(), "cargo run -- bin".bold(), r#""binary_number""#.green().bold());
            println!("{} {} {}", "DECIMAL TO BINARY:".yellow(), "cargo run -- dec".bold(), r#""decimal_number""#.green().bold());
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

fn hex_to_dec(mut input: String) -> i32
{
    let mut exp = 0;
    let mut sum: i32 = 0;

    while input.len() > 0
    {
        let last_digit = input.chars().last().unwrap();

        match last_digit
        {
            '0' => sum += 0,

            '1' => sum += i32::pow(16, exp),

            '2' => sum += 2 * i32::pow(16, exp),

            '3' => sum += 3 * i32::pow(16, exp),

            '4' => sum += 4 * i32::pow(16, exp),

            '5' => sum += 5 * i32::pow(16, exp),

            '6' => sum += 6 * i32::pow(16, exp),

            '7' => sum += 7 * i32::pow(16, exp),

            '8' => sum += 8 * i32::pow(16, exp),

            '9' => sum += 9 * i32::pow(16, exp),

            'A' => sum += 10 * i32::pow(16, exp),

            'B' => sum += 11 * i32::pow(16, exp),

            'C' => sum += 12 * i32::pow(16, exp),

            'D' => sum += 13 * i32::pow(16, exp),

            'E' => sum += 14 * i32::pow(16, exp),

            'F' => sum += 15 * i32::pow(16, exp),

            _ => panic!("you entered an invalid character!\nValid hexadecimal characters:\n0,1,2,3,4,5,6,7,8,9,A,B,C,D,E,F")
        };

        exp += 1;
        input.pop();
    }

    sum
}

fn dec_to_hex(mut input: i32) -> String
{
    let mut sum = String::new();
    
    while input > 0
    {
        let rem = input % 16;
        input /= 16;

            match rem
            {
                0 => sum.push('0'),
                
                1 => sum.push('1'),

                2 => sum.push('2'),

                3 => sum.push('3'),

                4 => sum.push('4'),

                5 => sum.push('5'),

                6 => sum.push('6'),

                7 => sum.push('7'),

                8 => sum.push('8'),

                9 => sum.push('9'),

                10 => sum.push('A'),

                11 => sum.push('B'),

                12 => sum.push('C'),

                13 => sum.push('D'),

                14 => sum.push('E'),

                15 => sum.push('F'),

                _ => panic!("something went wrong!")
            };

    }

    sum.chars().rev().collect::<String>()
}
