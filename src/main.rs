use std::io::Write;
use std::iter::Peekable;
use std::slice::Iter;
use std::str::Chars;

fn print_prompt() {
    print!("> ");
    std::io::stdout().flush().expect("Failed to flush stdout.");
}

fn read_expression(exp: &mut String) {
    exp.clear();
    std::io::stdin().read_line(exp).expect("Failed to read stdin.");
}

fn is_operator(ch: &char) -> bool {
    ch == &'+' || ch == &'-' || ch == &'*' || ch == &'/'
}

fn is_word_char(ch: &char) -> bool {
    ch.is_alphabetic() || ch.is_numeric() || is_operator(ch)
}

fn tokenize_word(chars: &mut Peekable<Chars>) -> String {
    let mut word = String::new();
    loop {
        match chars.peek() {
            Some(ch) => {
                if is_word_char(ch) {
                    word.push(*ch);
                    chars.next();
                } else {
                    break;
                }
            },
            None => break,
        }
    }

    word
}

fn tokenize(exp: &String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = exp.trim().chars().peekable();

    loop {
        match chars.peek() {
            Some(ch) => {
                if ch == &'(' || ch == &')' {
                    tokens.push(ch.to_string());
                    chars.next();
                } else if is_word_char(ch) {
                    tokens.push(tokenize_word(&mut chars));
                } else {
                    chars.next();
                }
            },
            None => break,
        }
    }

    tokens
}

fn eval_command(command: &String) {
    match command.trim() {
        ".exit" => {
            std::process::exit(0);
        },
        _ => {},
    }
}

struct AST {
    token: String,
    left: Option<String>,
    right: Option<String>,
}

impl AST {
    fn new(token: String) -> Self {
        Self {
            token,
            left: None,
            right: None,
        }
    }

    fn add_left(&mut self, left: String) {
        self.left = Some(left);
    }

    fn add_right(&mut self, right: String) {
        self.right = Some(right);
    }
}

fn parse_tokens(tokens: &mut Peekable<Iter<String>>) -> AST {
    let mut ast = AST::new(tokens.next().unwrap().to_string());

    ast.add_left(tokens.next().unwrap().to_string());
    ast.add_right(tokens.next().unwrap().to_string());

    ast
}

fn eval_ast(ast: &AST) {
    match ast.token.as_str() {
        "+" => {
            let left_val: u32 = ast.left.as_ref().unwrap().parse().unwrap();
            let right_val: u32 = ast.right.as_ref().unwrap().parse().unwrap();
            println!("{}", left_val + right_val);
        },
        _ => {},
    }
}

fn eval_expression(exp: &String) {
    let tokens = tokenize(exp);
    let mut tokens_iter = tokens.iter().peekable();

    tokens_iter.next();  // skip '('
    let ast = parse_tokens(&mut tokens_iter);
    eval_ast(&ast);
}

fn main() {
    let mut input = String::new();

    loop {
        print_prompt();
        read_expression(&mut input);

        if input.starts_with('.') {
            eval_command(&input);
        } else {
            eval_expression(&input);
        }
    }
}
