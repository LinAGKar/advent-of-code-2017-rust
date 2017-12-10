use std::io;
use std::io::Read;
use std::str::Chars;


#[derive(Debug)]
enum Token {
    Garbage(String),
    LBrace,
    RBrace,
    Comma,
    Invalid,
}


struct Tokens<'t> {
    chars: Chars<'t>,
}


impl<'t> Iterator for Tokens<'t> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next = self.chars.next();
        if let Some(next) = next {
            match next {
                '{' => Some(Token::LBrace),
                '}' => Some(Token::RBrace),
                ',' => Some(Token::Comma),
                '<' => {
                    let mut escaped = false;
                    let mut res = String::new();
                    loop {
                        let next = self.chars.next();
                        if let Some(next) = next {
                            if escaped {
                                escaped = false;
                            } else {
                                match next {
                                    '!' => {
                                        escaped = true;
                                    }
                                    '>' => {
                                        break;
                                    }
                                    _ => {
                                        res.push(next);
                                    }
                                }
                            }
                        } else {
                            return Some(Token::Invalid);
                        }
                    }
                    Some(Token::Garbage(res))
                }
                _ => Some(Token::Invalid),
            }
        } else {
            None
        }
    }
}


trait Tokenizer {
    fn tokens<'t>(&'t self) -> Tokens<'t>;
}


impl Tokenizer for str {
    fn tokens<'t>(&'t self) -> Tokens<'t> {
        Tokens {
            chars: self.chars(),
        }
    }
}



fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut level = 0;
    let mut total = 0;
    for i in input.tokens() {
        match i {
            Token::LBrace => {
                level += 1;
                total += level;
            }

            Token::RBrace => {
                level -= 1;
            }

            _ => {}
        }
    }
    println!("{}", total);
}
