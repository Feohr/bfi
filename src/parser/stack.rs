pub mod toks;
use toks::BfTokens;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The brackets are mismatched")]
    MismatchedBrackets,
    #[error("MismatchedLoop with no opening Loop token")]
    MismatchedLoop,
    #[error("The comment tag was not closed")]
    CommentTagNotClosed,
    #[error("Syntax error: {0}")]
    SyntaxError(#[from] SyntaxError),
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Only digits allowed but found {0:?}")]
    OnlyDigitsAllowedHere(char),
    #[error("Write the comment outside the bracket")]
    CommentInsideBracket,
    #[error("Cannot parse the numerical string")]
    ParseNumError,
    #[error("The paranthesis for repetative numbers are unclosed")]
    UnclosedParanthesis,
    #[error("Expected a symbol instead found {0}")]
    ExpectedSymbol(char),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BfWhile {
    End,
    Linear(char),
    Nested(Box<BfStackElem>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BfStackElem {
    Single(char),
    While(Vec<BfWhile>),
}

#[derive(Debug)]
pub struct BfStack {
    pub item: Vec<BfStackElem>,
    length: usize,
}

impl<'a> BfStack {
    // To initialize stack.
    #[inline]
    pub fn init_stack() -> Self {
        BfStack {
            item: Vec::new(),
            length: 0_usize,
        }
    }

    // Building a stack from string input.
    pub fn stackify(&mut self, source: &'a str) -> Result<(), Error> {

        let mut src: Vec<char> = source.chars().filter(|&x| is_useless(x)).collect();
        let mut pos: usize = 1_usize;
        std::mem::drop(source);

        loop {
            if src.is_empty() { break }
            match src[0] {
                BfTokens::FSL => {
                    // eating opening '/'
                    src.remove(0);
                    pos += 1;
                    while src[0] != BfTokens::FSL {
                        src.remove(0);
                        pos += 1;
                        if src.is_empty() { return Err(Error::CommentTagNotClosed) }
                    }
                },
                BfTokens::LLP => {
                    let loop_vec = read_loop(&mut src, &mut pos)?;
                    self.push_stack(BfStackElem::While(loop_vec));
                },
                BfTokens::RLP => return Err(Error::MismatchedLoop),
                BfTokens::LTB => {
                    let (action, mut repition) = read_rep(&mut src, &mut pos)?;
                    while repition > 0 {
                        self.push_stack(BfStackElem::Single(action));
                        repition -= 1;
                    }
                },
                BfTokens::RTB => return Err(Error::MismatchedBrackets),
                _ => self.push_stack(BfStackElem::Single(src[0])),
            }
            src.remove(0);
            pos += 1;
        }
        Ok(())
    }

    // Adding char as stack element.
    #[inline]
    fn push_stack(&mut self, elem: BfStackElem) {
        self.item.push(elem);
        self.length = self.item.len();
    }

    // To remove elements from the stack.
    #[inline]
    pub fn pop_stack(&mut self) {
        self.item.remove(0);
        self.length = self.item.len();
    }
}

// To read the loop chars and creating a vector element.
fn read_loop(source: &mut Vec<char>, pos: &mut usize) -> Result<Vec<BfWhile>, Error> {
    let mut loop_vec: Vec<BfWhile> = Vec::new();

    // Skipping the opening bracket.
    source.remove(0);
    *pos += 1;

    // Loop till we reach the closing bracket.
    loop {
        if source.is_empty() { return Err(Error::MismatchedLoop) }
        match source[0] {
            BfTokens::LLP => {
                // This is the internal loop.
                let nest_loop_vec = read_loop(source, pos)?;
                loop_vec.push(BfWhile::Nested(Box::new(BfStackElem::While(nest_loop_vec))));
            }
            BfTokens::RLP => {
                loop_vec.push(BfWhile::End);
                return Ok(loop_vec);
            }
            BfTokens::LTB => {
                let (action, mut repition) = read_rep(source, pos)?;
                while repition > 0 {
                    loop_vec.push(BfWhile::Linear(action));
                    repition -= 1;
                }
            }
            BfTokens::FSL => {
                // eating opening '/'
                source.remove(0);
                *pos += 1;
                while source[0] != BfTokens::FSL {
                    source.remove(0);
                    *pos += 1;
                    if source.is_empty() { return Err(Error::CommentTagNotClosed) }
                }
            }
            _ => {
                loop_vec.push(BfWhile::Linear(source[0]));
            }
        }
        source.remove(0);
        *pos += 1;
    }

}

// To read numbers
fn read_rep(source: &mut Vec<char>, pos: &mut usize) -> Result<(char, usize), SyntaxError> {
    // Skipping the opening loop.
    source.remove(0);
    *pos += 1;

    let mut number: String = String::new();
    let sign: char;

    match source[0] {
        BfTokens::ADD | BfTokens::SUB | BfTokens::LFT | BfTokens::RGT => {
            // Eating the symbol.
            sign = source[0];
            source.remove(0);
            *pos += 1;
        },
        _ => return Err(SyntaxError::ExpectedSymbol(source[0])),
    }

    loop {
        if source.is_empty() {
            return Err(SyntaxError::UnclosedParanthesis);
        }
        if !source[0].is_digit(10) {
            if source[0].eq(&BfTokens::RTB) {
                if number.is_empty() { return Ok((sign, 1)) }
                let Ok(num) = number
                    .trim()
                    .parse::<usize>() else {
                        return Err(SyntaxError::ParseNumError);
                };
                return Ok((sign, num));
            }
            if source[0].ne(&BfTokens::FSL) {
                return Err(SyntaxError::OnlyDigitsAllowedHere(source[0]));
            }
            return Err(SyntaxError::CommentInsideBracket);
        }
        number.push(source[0]);
        source.remove(0);
        *pos += 1;
    }

}

// To filter useless characters
#[inline]
fn is_useless(ch: char) -> bool {
    match ch {
        BfTokens::SPC | BfTokens::NXL | BfTokens::TAB => false,
        _ => true,
    }
}
