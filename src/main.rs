use std::env;
use std::io::{self, Write};
use clap::{Command, arg};
use itertools::Itertools;

fn main() -> io::Result<()> {
    let matches = Command::new("cypher-gen")
        .args(&[
            arg!(pangram: -p --pangram [PANGRAM] "Sets the pangram map used for the cypher (default a-z)"),
            arg!(separator: -s --separator [SEPARATOR] "Sets the separator used for the cypher (default '.')"),
            arg!(decrypt: -d --decrypt "Decrypts input of numbers to matching characters of pangram"),
            arg!(start_number: -n --"start-number" [NUMBER] "Sets the start number for the cypher (default 1)"),
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

    let default_number = "1".to_string();
    let number = matches.get_one::<String>("start_number").unwrap_or(&default_number);
    let number = number.parse::<isize>().unwrap();

    let output = if matches.is_present("decrypt") {
        let input_vec: Vec<i32> = input.split(separator).map(|x| x.parse::<i32>().unwrap()).collect();
        
        crate::decrypt_cypher(input_vec, pangram, number)
    } else {
        crate::to_cypher(input.as_str(), pangram, separator, number)
    };

    let mut stdout = io::stdout().lock();
    stdout.write_all(output.as_bytes()).unwrap();

    Ok(())
}

pub fn decrypt_cypher(input: Vec<i32>, pangram: &str, start_number: isize) -> String {
    let pangram = crate::unique_chars(pangram);

    let mut output = String::new();
    for i in input {
        let i = i as isize;
        output.push(pangram.chars().nth((crate::wrap_on_limit(i-start_number, 26)).try_into().unwrap()).unwrap());
    }

    return output;
}

fn unique_chars(pangram: &str) -> String {
    let pangram = pangram.to_lowercase();

    pangram.chars().into_iter().unique().filter(|x| !x.is_whitespace()).collect()
}

fn wrap_on_limit(index: isize, limit: isize) -> isize {
    let index = if index < 0 { index + limit } else { index };

    let output = index % limit;

    if output == 0 { limit } else { output }
}

pub fn to_cypher(input: &str, pangram: &str, separator: &str, start_number: isize) -> String {
    let pangram = crate::unique_chars(pangram);

    let mut output = String::new();

    let lower_input = input.to_lowercase();

    for c in lower_input.chars() {
        if c.is_alphabetic() {
            if !output.is_empty() {
                output.push_str(separator);
            }

            let index = pangram.find(c).unwrap() as isize;
            output.push_str((crate::wrap_on_limit(index+start_number, 26)).to_string().as_str());
        }
    }

    output
}

#[cfg(test)]
mod main_tests {
    #[test]
    fn to_cypher_generates_cypher() {
        let input = "Hello world";
        let pangram = "abcdefghijklmnopqrstuvwxyz";
        let separator = ".";

        let expected_output = "8.5.12.12.15.23.15.18.12.4";

        let output = crate::to_cypher(input, pangram, separator, 1);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn to_cypher_generates_cypher_from_custom_pangram() {
        let input = "Hello world";
        let pangram = "Sphinx of black quartz judge my vow";
        let separator = ".";

        let expected_output = "3.22.10.10.7.26.7.16.10.20";

        let output = crate::to_cypher(input, pangram, separator, 1);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn to_cypher_outputs_custom_separator_text() {
        let input = "Hello world";
        let pangram = "abcdefghijklmnopqrstuvwxyz";
        let separator = ", ";

        let expected_output = "8, 5, 12, 12, 15, 23, 15, 18, 12, 4";

        let output = crate::to_cypher(input, pangram, separator, 1);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn to_cypher_handles_uppercase_pangram() {
        let input = "Hello world";
        let pangram = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let separator = ", ";

        let expected_output = "8, 5, 12, 12, 15, 23, 15, 18, 12, 4";

        let output = crate::to_cypher(input, pangram, separator, 1);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn decrypt_cypher_to_text() {
        let input_numbers = vec![8, 5, 12, 12, 15, 23, 15, 18, 12, 4];
        let pangram = "abcdefghijklmnopqrstuvwxyz";

        let expected_output = "helloworld";

        let output = crate::decrypt_cypher(input_numbers, pangram, 1);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn decrypt_cypher_given_pangram_with_spaces_ignores_whitespace() {
        let input_numbers = vec![3, 22, 10, 10, 7, 26, 7, 16, 10, 20];
        let pangram = "Sphinx of black quartz judge my vow";

        let expected_output = "helloworld";

        let output = crate::decrypt_cypher(input_numbers, pangram, 1);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn decrypt_cypher_given_pangram_with_uppercase_assumes_lowercase() {
        let input_numbers = vec![8, 5, 12, 12, 15, 23, 15, 18, 12, 4];
        let pangram = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let expected_output = "helloworld";

        let output = crate::decrypt_cypher(input_numbers, pangram, 1);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn to_cypher_given_non_default_start_number_generates_cypher_with_that_number() {
        let input = "abcxyz";
        let pangram = "abcdefghijklmnopqrstuvwxyz";
        let separator = ".";
        let start_number = 4;

        let expected_output = "4.5.6.1.2.3";

        let output = crate::to_cypher(input, pangram, separator, start_number);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn decrypt_cypher_given_non_default_start_number_decypts_cypher_with_that_number_as_offset() {
        let input_numbers = vec![14, 12, 15, 3, 9, 1, 21];
        let pangram = "Sphinx of black quartz judge my vow";
        let start_number = 4;

        let expected_output = "abcwxyz";

        let output = crate::decrypt_cypher(input_numbers, pangram, start_number);

        assert_eq!(output, expected_output);
    }
}

