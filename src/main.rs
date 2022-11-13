#[derive(Debug)]
enum Token {
    LParen,
    RParen,
    Symbol(String),
    Integer(i64),
}

#[derive(Debug)]
enum Object {
    Symbol(String),
    Integer(i64),
    List(Vec<Object>),
}

fn parse_tokens(tokens: &mut Vec<Token>) -> Vec<Object> {
    let mut objects = Vec::new();

    loop {
        if tokens.len() < 1 { break; }

        let token = Some(tokens.remove(0));
        match token {
            Some(token) => {
                match token {
                    Token::Symbol(s) => objects.push(Object::Symbol(s.to_string())),
                    Token::Integer(i) => objects.push(Object::Integer(i)),
                    Token::LParen => {
                        objects.push(Object::List(parse_tokens(tokens)))
                    },
                    Token::RParen => break,
                    _ => {},
                }
            },
            None => panic!("Insufficien token"),
        }
    }

    objects
}

fn eval_symbol(sym_obj: &String) {
    println!("eval_symbol: {:?}", sym_obj);
}

fn eval_integer(int_obj: &i64) {
    println!("eval_integer: {:?}", int_obj);
}

fn eval_list(list_obj: &Vec<Object>) {
    println!("eval_list: {:?}", list_obj);
    for obj in list_obj {
        match obj {
            Object::Symbol(s) => eval_symbol(&s),
            Object::Integer(n) => eval_integer(&n),
            Object::List(l) => eval_list(&l),
        }
    }
}

fn eval(obj: &Object) {
    match obj {
        Object::Symbol(s) => eval_symbol(&s),
        Object::Integer(n) => eval_integer(&n),
        Object::List(l) => eval_list(&l),
    }
}

fn main() {
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).expect("Failed to read stdin.");

    text = text.trim().to_string()
        .replace("(", " ( ")
        .replace(")", " ) ");
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
    println!("tokens: {:?}", tokens);

    let objects = parse_tokens(&mut tokens);
    println!("objects: {:?}", objects);

    eval(&objects[0]);
}
