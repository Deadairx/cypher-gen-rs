use std::env;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut input = String::new();

    if args.len() == 1 || args[1] == "--help" {
        println!("Usage: {} [-p | --pangram <string>] <input>", args[0]);
        return Ok(());
    }

    let mut input_start = 1;

    let mut pangram = "abcdefghijklmnopqrstuvwxyz";

    if args[1] == "-p" || args[1] == "--pangram" {
        pangram = args[2].as_str();
        input_start = 3;
    }

    for arg in args[input_start..].iter() {
        input.push_str(&arg);
    }


    let output = crate::to_cypher(input.as_str(), pangram);

    let mut stdout = io::stdout().lock();
    stdout.write_all(output.as_bytes()).unwrap();

    Ok(())
}

pub fn to_cypher(input: &str, pangram: &str) -> String {
    let words: Vec<&str> = pangram.split_whitespace().collect();
    let cypher = words.join("");
    //let cypher = pangram.to_string();
    let mut output = String::new();

    let lower_input = input.to_lowercase();

    for c in lower_input.chars() {
        if c.is_alphabetic() {
            if !output.is_empty() {
                output.push_str(".");
            }

            let index = cypher.find(c).unwrap();
            output.push_str((index+1).to_string().as_str());
        }
    }

    output
}

#[cfg(test)]
mod main_tests {
    #[test]
    fn it_generates_cypher() {
        let input = "Hello world";
        let pangram = "abcdefghijklmnopqrstuvwxyz";

        let expected_output = "8.5.12.12.15.23.15.18.12.4";

        let output = crate::to_cypher(input, pangram);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn it_generates_cypher_from_custom_pangram() {
        let input = "Hello world";
        let pangram = "Sphinx of black quartz judge my vow";

        let expected_output = "3.24.10.10.7.29.7.17.10.22";

        let output = crate::to_cypher(input, pangram);

        assert_eq!(output, expected_output);
    }
}

