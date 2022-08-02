# cypher-gen

This cypher generator is based off 
[this reddit post](https://www.reddit.com/r/DnDBehindTheScreen/comments/txjloq/a_difficult_riddle_to_give_your_players_as/),
and is intended to be a simple Dungeon Master tool for some D&D puzzles.



`cypher-gen <string>` will output a number cypher based on `<string>` input

> (example: `"Hello, World!"` output: `8.5.12.12.15.23.15.18.12.4`)

add `-p --pangram <string>` to use custom pangram as source (default `a-z`)

> (example: `-p "Sphinx of black quartz judge my vow" "Hello, World!"` output: `3.22.10.10.7.26.7.16.10.20`)

add `-s --separator <string>` to change separator (default `.`)

> (example: `-s ", " "Hello, World!"` output: `8, 5, 12, 12, 15, 23, 15, 18, 12, 4`)

use `-d --decrypt` with input of numbers (separated by matching separator) to decrypt back into letters

> (example: `-d  "8.5.12.12.15.23.15.18.12.4"` output: `helloworld`)

