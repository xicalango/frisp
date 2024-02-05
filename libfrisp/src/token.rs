
use std::fmt::Debug;

#[derive(Debug)]
pub enum Token {
    ListStart,
    ListEnd,
    Integer(isize),
    Float(f64),
    Symbol(String),
    String(String),
    Error(String),
}

pub struct TokenStream<I> {
    iter: I,
    next_token: Option<Token>,
}

impl<I> TokenStream<I> {

    pub fn new(iter: I) -> TokenStream<I> {
        TokenStream {
            iter,
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
                    return Some(Token::Error("EOF while reading string".to_string()));
                },
                c if c == '-' || c.is_ascii_digit() => {
                    let mut is_float = false;

                    let mut buf = String::new();
                    buf.push(c);

                    while let Some(c) = self.iter.next() {
                        if c.is_whitespace() {
                            break;
                        } else if c == ')' {
                            self.next_token.replace(Token::ListEnd);
                            break;
                         } else if c.is_ascii_digit() {
                            buf.push(c);
                        } else if c == '.' {
                            is_float = true;
                            buf.push(c);
                        } else {
                            return Some(Token::Error(format!("invalid digit: {c}")));
                        }
                    }

                    if is_float {
                        return Some(Token::Float(buf.parse().unwrap()));
                    } else {
                        return Some(Token::Integer(buf.parse().unwrap()));
                    }
                }
                c if c.is_ascii_alphabetic() => {
                    let mut buf = String::new();
                    buf.push(c);
                    while let Some(c) = self.iter.next() {
                        if c.is_whitespace() {
                            break;
                        } else if c == '(' {
                            self.next_token.replace(Token::ListStart);
                            break;
                        } else if c == ')' {
                            self.next_token.replace(Token::ListEnd);
                            break;
                        } else if c.is_ascii_alphanumeric() {
                            buf.push(c)
                        } else {
                            return Some(Token::Error(format!("invalid char sym: {c}")));
                        }
                    }

                    return Some(Token::Symbol(buf));
                }
                e => return Some(Token::Error(format!("invalid token: {e:?}"))),
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
