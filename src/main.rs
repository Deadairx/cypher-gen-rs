use std::env;
use std::io::{self, Write};
use clap::{Command, arg};

fn main() -> io::Result<()> {
    let matches = Command::new("cypher-gen")
        .args(&[
            arg!(pangram: -p --pangram [PANGRAM] "Sets the pangram map used for the cypher (default a-z)"),
            arg!(separator: -s --separator [SEPARATOR] "Sets the separator used for the cypher (default '.')"),
            arg!(<INPUT> "The input text to be encrypted"),
        ])
        .get_matches();

    let args: Vec<String> = env::args().collect();

    let mut input = String::new();

    if args.len() == 1 || args[1] == "--help" {
        println!("Usage: {} [-p | --pangram <string>] <input>", args[0]);
        return Ok(());
    }

    let mut pangram = "abcdefghijklmnopqrstuvwxyz";

    if let Some(p) = matches.get_one::<String>("pangram") {
        pangram = p;
    }

    println!("Pangram: {}", &pangram);

    let mut separator = ".";

    if let Some(s) = matches.get_one::<String>("separator") {
        separator = s;
    }

    if let Some(i) = matches.get_one::<String>("INPUT") {
        input = i.to_string();
    }

    let output = crate::to_cypher(input.as_str(), pangram, separator);

    let mut stdout = io::stdout().lock();
    stdout.write_all(output.as_bytes()).unwrap();

    Ok(())
}

pub fn to_cypher(input: &str, pangram: &str, separator: &str) -> String {
    let words: Vec<&str> = pangram.split_whitespace().collect();
    let cypher = words.join("").to_lowercase();
    //let cypher = pangram.to_string();
    let mut output = String::new();

    let lower_input = input.to_lowercase();

    for c in lower_input.chars() {
        if c.is_alphabetic() {
            if !output.is_empty() {
                output.push_str(separator);
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
        let separator = ".";

        let expected_output = "8.5.12.12.15.23.15.18.12.4";

        let output = crate::to_cypher(input, pangram, separator);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn it_generates_cypher_from_custom_pangram() {
        let input = "Hello world";
        let pangram = "Sphinx of black quartz judge my vow";
        let separator = ".";

        let expected_output = "3.24.10.10.7.29.7.17.10.22";

        let output = crate::to_cypher(input, pangram, separator);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn it_outputs_custom_separator_text() {
        let input = "Hello world";
        let pangram = "abcdefghijklmnopqrstuvwxyz";
        let separator = ", ";

        let expected_output = "8, 5, 12, 12, 15, 23, 15, 18, 12, 4";

        let output = crate::to_cypher(input, pangram, separator);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn it_handles_uppercase_pangram() {
        let input = "Hello world";
        let pangram = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let separator = ", ";

        let expected_output = "8, 5, 12, 12, 15, 23, 15, 18, 12, 4";

        let output = crate::to_cypher(input, pangram, separator);

        assert_eq!(output, expected_output);
    }
}

