use std::env;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut input = String::new();
    for arg in args[1..].iter() {
        input.push_str(&arg);
    }

    let output = crate::to_cypher(input.as_str());

    let mut stdout = io::stdout().lock();
    stdout.write_all(output.as_bytes()).unwrap();

    Ok(())
}

pub fn to_cypher(input: &str) -> String {
    let default_cypher = "abcdefghijklmnopqrstuvwxyz";
    let cypher = default_cypher.to_string();
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

        let expected_output = "8.5.12.12.15.23.15.18.12.4";

        let output = crate::to_cypher(input);

        assert_eq!(output, expected_output);
    }
}

