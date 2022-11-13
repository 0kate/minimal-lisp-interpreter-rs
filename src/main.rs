use std::collections::HashMap;

#[derive(Debug)]
enum Token {
    LParen,
    RParen,
    Symbol(String),
    Integer(i64),
}

#[derive(Debug, Clone)]
enum Object {
    Void,
    Symbol(String),
    Integer(i64),
    List(Vec<Object>),
    Lambda(Vec<String>, Vec<Object>),
}

fn parse_tokens(tokens: &mut Vec<Token>) -> Vec<Object> {
    let mut objects = Vec::new();

    loop {
        if tokens.len() < 1 { break; }

        let token = tokens.remove(0);
        match token {
            Token::Symbol(s) => objects.push(Object::Symbol(s.to_string())),
            Token::Integer(i) => objects.push(Object::Integer(i)),
            Token::LParen => {
                objects.push(Object::List(parse_tokens(tokens)))
            },
            Token::RParen => break,
            _ => {},
        }
    }

    objects
}

#[derive(Debug)]
struct Env {
    vars: HashMap<String, Object>,
}

impl Env {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    fn set(&mut self, name: &str, val: Object) {
        self.vars.insert(name.to_string(), val);
    }
}

fn tokenize(text: String) -> Vec<Token> {
    let words = text.split_whitespace();

    let mut tokens = Vec::new();
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                if let Ok(num) = word.parse::<i64>() {
                    tokens.push(Token::Integer(num));
                } else {
                    tokens.push(Token::Symbol(word.to_string()));
                }
            },
        }
    }

    tokens
}

struct VirtualMachine {
    env: Env,
}

impl VirtualMachine {
    fn new() -> Self {
        Self {
            env: Env::new(),
        }
    }

    fn eval_define(&mut self, list: &Vec<Object>) -> Object {
        let sym = match &list[1] {
            Object::Symbol(s) => s.clone(),
            _ => String::new(),
        };

        let val = self.eval(&list[2]);
        self.env.set(&sym, val);

        Object::Void
    }

    fn eval_lambda(&mut self, list: &Vec<Object>) -> Object {
        let params = if let Object::List(list) = &list[1] {
            let mut params = Vec::new();
            for param in list {
                if let Object::Symbol(s) = param {
                    params.push(s.clone());
                } else {
                    panic!("Invalid lambda params");
                }
            }

            params
        } else {
            panic!("Invalid lambda");
        };

        let body = if let Object::List(list) = &list[2] {
            list.clone()
        } else {
            panic!("Invalid lambda");
        };

        Object::Lambda(params, body)
    }

    fn eval_list(&mut self, list: &Vec<Object>) -> Object {
        println!("eval_list: {:?}", list);

        if let Some(obj) = list.first() {
            match obj {
                Object::Symbol(s) => match s.as_str() {
                    "define" => self.eval_define(&list),
                    "lambda" => self.eval_lambda(&list),
                    _ => Object::Void,
                },
                _ => Object::Void,
            }
        } else {
            Object::Void
        }
    }

    fn eval(&mut self, obj: &Object) -> Object {
        match obj {
            // Object::Symbol(s) => eval_symbol(&s),
            // Object::Integer(n) => eval_integer(&n),
            Object::List(l) => self.eval_list(l),
            _ => Object::Void,
        }
    }
}

fn main() {
    let mut text = String::new();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Failed to read stdin.");

    text = text.trim().to_string()
        .replace("(", " ( ")
        .replace(")", " ) ");

    let mut tokens = tokenize(text);
    println!("tokens: {:?}", tokens);

    let ast = parse_tokens(&mut tokens);
    println!("ast: {:?}", ast);

    let mut vm = VirtualMachine::new();
    let _ = vm.eval(ast.first().unwrap());
    println!("{:?}", vm.env);
}
