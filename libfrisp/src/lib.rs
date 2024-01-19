use std::{fmt::Debug, iter::Peekable};


#[derive(Debug)]
pub enum Token {
    ListStart,
    ListEnd,
    Integer(isize),
    Float(f64),
    Symbol(String),
    String(String),
}

#[derive(Debug)]
enum TokenizerState {
    Normal,
    String,
}

pub struct TokenStream<I> 
where I: Iterator<Item = char> {
    iter: Peekable<I>,
    next_token: Option<Token>,
}

impl<I> TokenStream<I>
where I: Iterator<Item = char> {

    pub fn new(iter: I) -> TokenStream<I> {
        TokenStream {
            iter: iter.peekable(),
            next_token: None,
        }
    }

}

impl<I> Debug for TokenStream<I>
where I: Debug + Iterator<Item = char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenStream").field("iter", &self.iter).finish()
    }
}

impl<I> Iterator for TokenStream<I> 
where I: Iterator<Item = char> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.next_token.take() {
            return Some(t);
        }

        while let Some(c) = self.iter.next() {
            match c {
                w if w.is_whitespace() => continue,
                '(' => return Some(Token::ListStart),
                ')' => return Some(Token::ListEnd),
                '"' => {
                    let mut buf = String::new();
                    while let Some(c) = self.iter.next() {
                        if c == '"' {
                            return Some(Token::String(buf));
                        }

                        buf.push(c);
                    }
                    panic!("EOF while reading string");
                },
                c if c.is_ascii_digit() => {
                    let mut is_float = false;

                    let mut buf = String::new();
                    buf.push(c);

                    while let Some(c) = self.iter.next() {
                        if c.is_whitespace() {
                            if is_float {
                                return Some(Token::Float(buf.parse().unwrap()));
                            } else {
                                return Some(Token::Integer(buf.parse().unwrap()));
                            }
                        } else if c == ')' {
                            self.next_token.replace(Token::ListEnd);

                            if is_float {
                                return Some(Token::Float(buf.parse().unwrap()));
                            } else {
                                return Some(Token::Integer(buf.parse().unwrap()));
                            }
                         } else if c.is_ascii_digit() {
                            buf.push(c);
                        } else if c == '.' {
                            is_float = true;
                            buf.push(c);
                        } else {
                            panic!("invalid digit: {c}");
                        }
                    }

                    panic!("EOF while reading number");
                }
                c if c.is_ascii_alphabetic() => {
                    let mut buf = String::new();
                    buf.push(c);
                    while let Some(c) = self.iter.next() {
                        if c.is_whitespace() {
                            return Some(Token::Symbol(buf));
                        } else if c == '(' {
                            self.next_token.replace(Token::ListStart);
                            return Some(Token::Symbol(buf));
                        } else if c == ')' {
                            self.next_token.replace(Token::ListEnd);
                            return Some(Token::Symbol(buf));
                        } else if c.is_ascii_alphanumeric() {
                            buf.push(c)
                        } else {
                            panic!("invalid symbol char: {c}");
                        }
                    }

                    panic!("EOF while reading symbol");
                }
                e => panic!("eheh {e:?}"),
            }
        }
        return None;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let stream = TokenStream::new("(cdr(1)(2)(\"asd\"))".chars());

        for t in stream {
            println!("{t:?}");
        }

    }
}
