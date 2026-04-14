use std::env;
use std::fs;
use std::path::Path;

// Main compiler struct
// Stores the input characters, current position, current character,
// and all tokens found during lexical analysis.
struct MyCompiler {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
    tokens: Vec<String>,
}

impl MyCompiler {
    // Creates a new compiler from the source text
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

    // Advances to the next character in the input
    fn advance(&mut self) {
        self.position += 1;
        if self.position < self.input.len() {
            self.current_char = Some(self.input[self.position]);
        } else {
            self.current_char = None;
        }
    }

    // Skips over whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    // Checks if a token is one of the valid LOLCODE keywords/tags
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

    // Gets the next token from the input character-by-character
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

        // Handle numbers
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

        // Handle punctuation or single-character text
        self.advance();
        c.to_string()
    }

    // Main compilation process:
    // 1. Perform lexical analysis
    // 2. Store tokens
    // 3. Run syntax analysis
    // 4. Generate HTML if syntax is valid
    fn compile(&mut self, input_filename: &str) {
        println!("Starting lexical analysis...");

        loop {
            let token = self.get_next_token();
            if token == "EOF" {
                break;
            }

            // Classify the token as valid keyword or normal text
            if token.starts_with('#') || token.chars().all(|c| c.is_alphanumeric()) {
                if self.lookup(&token) {
                    println!("VALID TOKEN: {}", token);
                } else {
                    println!("TEXT/IDENTIFIER: {}", token);
                }
            } else {
                println!("TEXT: {}", token);
            }

            // Save token for parsing later
            self.tokens.push(token);
        }

        println!("Lexical analysis complete.");

        // Only continue if syntax analysis passes
        if self.parse() {
            self.generate_html(input_filename);
        }
    }

    // Performs basic syntax analysis on the token list
    fn parse(&self) -> bool {
        println!("\nStarting syntax analysis...");

        // Check for empty input
        if self.tokens.is_empty() {
            println!("Syntax Error: input is empty.");
            return false;
        }

        // Check that file starts with #HAI
        if self.tokens.first().map(String::as_str) != Some("#HAI") {
            println!("Syntax Error: file must start with #HAI");
            return false;
        }

        // Check that file ends with #KBYE
        if self.tokens.last().map(String::as_str) != Some("#KBYE") {
            println!("Syntax Error: file must end with #KBYE");
            return false;
        }

        let mut i = 0;

        // Walk through tokens and validate block structure
        while i < self.tokens.len() {
            match self.tokens[i].as_str() {
                // Check comment block
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

                // Check MAEK blocks
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

                // Check GIMMEH blocks
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

    // Generates HTML output from the token list
    fn generate_html(&self, input_filename: &str) {
        println!("\nGenerating HTML...");

        let mut html = String::new();
        let mut i = 0;

        while i < self.tokens.len() {
            match self.tokens[i].as_str() {
                // Start and end of HTML document
                "#HAI" => html.push_str("<html>\n<body>\n"),
                "#KBYE" => html.push_str("</body>\n</html>\n"),

                // Comment block
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

                // Handle MAEK blocks
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

                // Close blocks
                "#MKAY" => {
                    if html.ends_with("<head>\n") {
                        html.push_str("</head>\n");
                    } else if html.ends_with("<ul>\n") || html.contains("<li>") {
                        html.push_str("</ul>\n");
                    } else {
                        html.push_str("</p>\n");
                    }
                }

                // Handle GIMMEH blocks
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

                // Handle newline
                "#NEWLINE" => html.push_str("<br>\n"),

                // Handle regular text
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

        // Create output filename with .html extension
        let output_filename = Path::new(input_filename)
            .with_extension("html");

        // Write HTML output file
        match fs::write(&output_filename, html) {
            Ok(_) => println!("HTML file generated: {}", output_filename.display()),
            Err(err) => println!("Failed to write HTML file: {}", err),
        }
    }
}

// Program entry point
fn main() {
    let args: Vec<String> = env::args().collect();

    // Check that an input file was provided
    if args.len() < 2 {
        println!("Usage: cargo run <file.lol>");
        return;
    }

    let filename = &args[1];

    // Check that the file has the correct extension
    if !filename.ends_with(".lol") {
        println!("Error: Must be a .lol file");
        return;
    }

    // Read the source file
    let contents = fs::read_to_string(filename)
        .expect("Failed to read file");

    // Create compiler and start compilation
    let mut compiler = MyCompiler::new(&contents);
    compiler.compile(filename);
}