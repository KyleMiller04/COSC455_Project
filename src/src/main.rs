use std::env;
use std::fs;

struct MyCompiler {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl MyCompiler {
    fn new(source: &str) -> Self {
        let chars: Vec<char> = source.chars().collect();
        let first_char = chars.get(0).cloned();

        Self {
            input: chars,
            position: 0,
            current_char: first_char,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        if self.position < self.input.len() {
            self.current_char = Some(self.input[self.position]);
        } else {
            self.current_char = None;
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn get_next_token(&mut self) -> String {
        self.skip_whitespace();

        if let Some(c) = self.current_char {
            if c.is_alphabetic() {
                let mut result = String::new();

                while let Some(ch) = self.current_char {
                    if ch.is_alphanumeric() {
                        result.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                return result;
            }

            self.advance();
            return c.to_string();
        }

        "EOF".to_string()
    }

    fn compile(&mut self) {
        println!("Starting lexical analysis...");

        loop {
            let token = self.get_next_token();
            if token == "EOF" {
                break;
            }

            println!("Token: {}", token);
        }

        println!("Done.");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <file.lol>");
        return;
    }

    let filename = &args[1];

    if !filename.ends_with(".lol") {
        println!("Error: Must be a .lol file");
        return;
    }

    let contents = fs::read_to_string(filename)
        .expect("Failed to read file");

    let mut compiler = MyCompiler::new(&contents);
    compiler.compile();
}