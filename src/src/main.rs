use std::env;
use std::fs;

struct MyCompiler {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    tokens: Vec<String>,
}

impl MyCompiler {
    fn new(source: &str) -> Self {
        let chars: Vec<char> = source.chars().collect();
        let first_char = chars.first().cloned();

        Self {
            input: chars,
            position: 0,
            current_char: first_char,
            tokens: Vec::new(),
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

    fn lookup(&self, token: &str) -> bool {
        matches!(
            token,
            "#HAI"
                | "#KBYE"
                | "#OBTW"
                | "#TLDR"
                | "#MAEK"
                | "#MKAY"
                | "#GIMMEH"
                | "#OIC"
                | "#NEWLINE"
                | "#IHAZ"
                | "#ITIZ"
                | "#LEMMESEE"
                | "HEAD"
                | "TITLE"
                | "PARAGRAF"
                | "BOLD"
                | "ITALICS"
                | "LIST"
                | "ITEM"
                | "LINX"
        )
    }

    fn get_next_token(&mut self) -> String {
        self.skip_whitespace();

        let Some(c) = self.current_char else {
            return "EOF".to_string();
        };

        // Handle LOLCODE tags like #HAI, #KBYE, #MAEK, etc.
        if c == '#' {
            let mut result = String::new();
            result.push(c);
            self.advance();

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

        // Handle words like HEAD, TITLE, PARAGRAF, etc.
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

        // Handle punctuation or single characters as text
        self.advance();
        c.to_string()
    }

    fn compile(&mut self) {
        println!("Starting lexical analysis...");

        loop {
            let token = self.get_next_token();
            if token == "EOF" {
                break;
            }

            if token.starts_with('#') || token.chars().all(|c| c.is_alphanumeric()) {
                if self.lookup(&token) {
                    println!("VALID TOKEN: {}", token);
                } else {
                    println!("TEXT/IDENTIFIER: {}", token);
                }
            } else {
                println!("TEXT: {}", token);
            }

            self.tokens.push(token);
        }

        println!("Lexical analysis complete.");

        self.parse();
    }

    fn parse(&self) {
        println!("\nStarting simple syntax check...");

        if self.tokens.is_empty() {
            println!("Syntax Error: input is empty.");
            return;
        }

        if self.tokens.first().map(String::as_str) != Some("#HAI") {
            println!("Syntax Error: file must start with #HAI");
            return;
        }

        if self.tokens.last().map(String::as_str) != Some("#KBYE") {
            println!("Syntax Error: file must end with #KBYE");
            return;
        }

        println!("Basic syntax check passed.");
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