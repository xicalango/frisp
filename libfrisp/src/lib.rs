use std::fmt::Debug;


#[derive(Debug)]
pub enum Token {
    ListStart,
    ListEnd,
    Integer(isize),
    Float(f64),
    Symbol(String),
}

#[derive(Debug)]
enum TokenizerState {
    Normal,
    String,
}

pub struct TokenStream<I> {
    iter: I,
}

impl<I> TokenStream<I> {

    pub fn new(iter: I) -> TokenStream<I> {
        TokenStream {
            iter
        }
    }

}

impl<I> Debug for TokenStream<I>
where I: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenStream").field("iter", &self.iter).finish()
    }
}

impl<I> Iterator for TokenStream<I> 
where I: Iterator<Item = char> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some('(') => Some(Token::ListStart),
            Some(')') => Some(Token::ListEnd),
            None => None,
            e => panic!("eheh {e:?}"),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let stream = TokenStream::new("(())".chars());

        for t in stream {
            println!("{t:?}");
        }

    }
}
