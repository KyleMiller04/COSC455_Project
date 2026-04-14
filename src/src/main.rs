use std::env;
use std::fs;
use std::path::Path;

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

        if c.is_numeric() {
            let mut result = String::new();

            while let Some(ch) = self.current_char {
                if ch.is_numeric() {
                    result.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }

            return result;
        }

        self.advance();
        c.to_string()
    }

    fn compile(&mut self, input_filename: &str) {
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

        if self.parse() {
            self.generate_html(input_filename);
        }
    }

    fn parse(&self) -> bool {
        println!("\nStarting syntax analysis...");

        if self.tokens.is_empty() {
            println!("Syntax Error: input is empty.");
            return false;
        }

        if self.tokens.first().map(String::as_str) != Some("#HAI") {
            println!("Syntax Error: file must start with #HAI");
            return false;
        }

        if self.tokens.last().map(String::as_str) != Some("#KBYE") {
            println!("Syntax Error: file must end with #KBYE");
            return false;
        }

        let mut i = 0;

        while i < self.tokens.len() {
            match self.tokens[i].as_str() {
                "#OBTW" => {
                    i += 1;
                    while i < self.tokens.len() && self.tokens[i] != "#TLDR" {
                        i += 1;
                    }
                    if i >= self.tokens.len() {
                        println!("Syntax Error: comment started with #OBTW but missing #TLDR");
                        return false;
                    }
                }

                "#MAEK" => {
                    if i + 1 < self.tokens.len() && self.tokens[i + 1] == "HEAD" {
                        i += 2;
                        while i < self.tokens.len() && self.tokens[i] != "#MKAY" {
                            i += 1;
                        }
                        if i >= self.tokens.len() {
                            println!("Syntax Error: HEAD block missing #MKAY");
                            return false;
                        }
                    } else if i + 1 < self.tokens.len() && self.tokens[i + 1] == "PARAGRAF" {
                        i += 2;
                        while i < self.tokens.len() && self.tokens[i] != "#MKAY" {
                            i += 1;
                        }
                        if i >= self.tokens.len() {
                            println!("Syntax Error: PARAGRAF block missing #MKAY");
                            return false;
                        }
                    } else if i + 1 < self.tokens.len() && self.tokens[i + 1] == "LIST" {
                        i += 2;
                        while i < self.tokens.len() && self.tokens[i] != "#MKAY" {
                            i += 1;
                        }
                        if i >= self.tokens.len() {
                            println!("Syntax Error: LIST block missing #MKAY");
                            return false;
                        }
                    }
                }

                "#GIMMEH" => {
                    if i + 1 < self.tokens.len() && self.tokens[i + 1] == "TITLE" {
                        i += 2;
                        while i < self.tokens.len() && self.tokens[i] != "#OIC" {
                            i += 1;
                        }
                        if i >= self.tokens.len() {
                            println!("Syntax Error: TITLE block missing #OIC");
                            return false;
                        }
                    } else if i + 1 < self.tokens.len()
                        && (self.tokens[i + 1] == "BOLD"
                            || self.tokens[i + 1] == "ITALICS"
                            || self.tokens[i + 1] == "ITEM"
                            || self.tokens[i + 1] == "LINX")
                    {
                        i += 2;
                        while i < self.tokens.len() && self.tokens[i] != "#OIC" {
                            i += 1;
                        }
                        if i >= self.tokens.len() {
                            println!("Syntax Error: GIMMEH block missing #OIC");
                            return false;
                        }
                    }
                }

                _ => {}
            }

            i += 1;
        }

        println!("Syntax analysis passed.");
        true
    }

    fn generate_html(&self, input_filename: &str) {
        println!("\nGenerating HTML...");

        let mut html = String::new();
        let mut i = 0;

        while i < self.tokens.len() {
            match self.tokens[i].as_str() {
                "#HAI" => html.push_str("<html>\n<body>\n"),
                "#KBYE" => html.push_str("</body>\n</html>\n"),

                "#OBTW" => {
                    html.push_str("<!-- ");
                    i += 1;
                    while i < self.tokens.len() && self.tokens[i] != "#TLDR" {
                        html.push_str(&self.tokens[i]);
                        html.push(' ');
                        i += 1;
                    }
                    html.push_str("-->\n");
                }

                "#MAEK" => {
                    if i + 1 < self.tokens.len() {
                        match self.tokens[i + 1].as_str() {
                            "HEAD" => {
                                html.push_str("<head>\n");
                                i += 1;
                            }
                            "PARAGRAF" => {
                                html.push_str("<p>");
                                i += 1;
                            }
                            "LIST" => {
                                html.push_str("<ul>\n");
                                i += 1;
                            }
                            _ => {}
                        }
                    }
                }

                "#MKAY" => {
                    if html.ends_with("<head>\n") {
                        html.push_str("</head>\n");
                    } else if html.ends_with("<ul>\n") || html.contains("<li>") {
                        html.push_str("</ul>\n");
                    } else {
                        html.push_str("</p>\n");
                    }
                }

                "#GIMMEH" => {
                    if i + 1 < self.tokens.len() {
                        match self.tokens[i + 1].as_str() {
                            "TITLE" => {
                                html.push_str("<title>");
                                i += 2;
                                while i < self.tokens.len() && self.tokens[i] != "#OIC" {
                                    html.push_str(&self.tokens[i]);
                                    html.push(' ');
                                    i += 1;
                                }
                                html.push_str("</title>\n");
                            }
                            "BOLD" => {
                                html.push_str("<b>");
                                i += 2;
                                while i < self.tokens.len() && self.tokens[i] != "#OIC" {
                                    html.push_str(&self.tokens[i]);
                                    html.push(' ');
                                    i += 1;
                                }
                                html.push_str("</b>");
                            }
                            "ITALICS" => {
                                html.push_str("<i>");
                                i += 2;
                                while i < self.tokens.len() && self.tokens[i] != "#OIC" {
                                    html.push_str(&self.tokens[i]);
                                    html.push(' ');
                                    i += 1;
                                }
                                html.push_str("</i>");
                            }
                            "ITEM" => {
                                html.push_str("<li>");
                                i += 2;
                                while i < self.tokens.len() && self.tokens[i] != "#OIC" {
                                    html.push_str(&self.tokens[i]);
                                    html.push(' ');
                                    i += 1;
                                }
                                html.push_str("</li>\n");
                            }
                            "LINX" => {
                                i += 2;
                                if i < self.tokens.len() && self.tokens[i] != "#OIC" {
                                    let url = self.tokens[i].clone();
                                    html.push_str(&format!("<a href=\"{0}\">{0}</a>", url));
                                    while i < self.tokens.len() && self.tokens[i] != "#OIC" {
                                        i += 1;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                "#NEWLINE" => html.push_str("<br>\n"),

                token => {
                    if !token.starts_with('#')
                        && token != "HEAD"
                        && token != "TITLE"
                        && token != "PARAGRAF"
                        && token != "BOLD"
                        && token != "ITALICS"
                        && token != "LIST"
                        && token != "ITEM"
                        && token != "LINX"
                    {
                        html.push_str(token);
                        html.push(' ');
                    }
                }
            }

            i += 1;
        }

        let output_filename = Path::new(input_filename)
            .with_extension("html");

        match fs::write(&output_filename, html) {
            Ok(_) => println!("HTML file generated: {}", output_filename.display()),
            Err(err) => println!("Failed to write HTML file: {}", err),
        }
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
    compiler.compile(filename);
}