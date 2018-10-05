#![no_std]

extern crate core as std;

extern crate cpuio;
extern crate hashmap_core;
extern crate ux;

pub const ERR: i2 = 0;
pub const FILE: &str = "empty";
pub const TARGET: &str = ":c//";

// TODO:
// Implement long_token
macro_rules! lex {
    () => {{
        let mut comment: i2;
        let mut token_length: i4;
        let mut tokens: &str;

        loop {
            if comment == 0 {
                let mut token = match char {
                    // insert code fold
                    " " => "space",
                    "(" => "begin_print",
                    ")" => "end_print",
                    "{" => "begin",
                    "}" => "print",

                    "=" => "equal",
                    "+" => "equal",

                    "1" => "int",
                    "2" => "int",
                    "3" => "int",
                    "4" => "int",
                    "5" => "int",
                    "6" => "int",
                    "7" => "int",
                    "8" => "int",
                    "9" => "int",
                    "0" => "int",

                    "a" => "char",
                    "b" => "char",
                    "c" => "char",
                    "d" => "char",
                    "e" => "char",
                    "f" => "char",
                    "g" => "char",
                    "h" => "char",
                    "i" => "char",
                    "j" => "char",
                    "k" => "char",
                    "l" => "char",
                    "m" => "char",
                    "n" => "char",
                    "o" => "char",
                    "p" => "char",
                    "q" => "char",
                    "r" => "char",
                    "s" => "char",
                    "t" => "char",
                    "u" => "char",
                    "v" => "char",
                    "w" => "char",
                    "y" => "char",
                    "z" => "char",

                    // end code fold
                }
            }

            token_length = token_length + 1;

            // cry havoc and let loose the ifs of war
            if token_length > 3 {
                if tokens == "strings" {
                    let id = match long_token {
                        // syntax
                        "call" => "syntax",
                        "extern" => "syntax",
                        "func" => "syntax",
                        "id" => "syntax",
                        "if" => "syntax",
                        "local" => "syntax",
                        "print" => "syntax",
                        "println" => "syntax",
                        "struct" => "syntax",
                        "thread" => "syntax",
                        "use" => "syntax",
                        "while" => "syntax",
                        "vga" => "syntax",

                        "//" => "comment",
                        _ => {
                            if next_line == "{" {
                                
                            }
                        },
                    }
                }

                if tokens == "ints" { break; }
            }

            if token == "int" {
                int.push_str(char);
            }

            if token == "char" {
                arb_str.push_str(char);
            }

            if token == ";" {
                match prev_token {
                    "int" => {
                        int.push_str(";");
                    }
                    "char" => {
                        arb_str.push_str(";");
                    }
                    "comment" => {
                        comment = 1;
                    }
                    "syntax" => {
                        logic.push_str(";");
                    }
                }
            }
        }
    }
