
use std::fmt::Debug;

use crate::Error;

#[derive(Debug)]
pub enum Token {
    ListStart,
    ListEnd,
    String(String),
    Symbol(String),
}

pub struct TokenStream<I> {
    iter: I,
    next_token: Option<Token>,
}

pub trait FrispSymbolChar {
    fn is_frisp_symbol(&self) -> bool;
}

impl FrispSymbolChar for char {
    fn is_frisp_symbol(&self) -> bool {
        match self {
            '(' | ')' => false,
            c if c.is_ascii_alphanumeric() => true,
            c if c.is_ascii_punctuation() => true,
            _ => false,
        }
    }
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
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.next_token.take() {
            return Some(Ok(t));
        }

        while let Some(c) = self.iter.next() {
            match c {
                w if w.is_whitespace() => continue,
                '#' => {
                    while let Some(c) = self.iter.next() {
                        if c.is_ascii_control() {
                            break;
                        }
                    }
                }
                '(' => return Some(Ok(Token::ListStart)),
                ')' => return Some(Ok(Token::ListEnd)),
                '"' => {
                    let mut buf = String::new();
                    while let Some(c) = self.iter.next() {
                        if c == '"' {
                            return Some(Ok(Token::String(buf)));
                        }

                        if c == '\\' {
                            if let Some(next) = self.iter.next() {
                                buf.push(next);
                                continue;
                            } else {
                                break;
                            }
                        }

                        buf.push(c);
                    }
                    return Some(Err(Error::TokenizerError("EOF while reading string".to_string())));
                },
                c if c.is_frisp_symbol() => {
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
                        } else if c.is_frisp_symbol() {
                            buf.push(c)
                        } else {
                            return Some(Err(Error::TokenizerError(format!("invalid char sym: {c}"))));
                        }
                    }

                    return Some(Ok(Token::Symbol(buf)));
                }
                e => return Some(Err(Error::TokenizerError(format!("invalid token: {e:?}")))),
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
