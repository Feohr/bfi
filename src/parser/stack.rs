#[macro_use]
pub mod toks;
use toks::BfTokens;
// use std::collections::HashMap;

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
    pub fn stackify(&mut self, source: &'a str) {

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
                        if src.is_empty() {
                            bfpanic!("Error", "Comment tag not closed", BfTokens::FSL, true);
                        }
                    }
                },
                BfTokens::LLP => {
                    let loop_vec = read_loop(&mut src, &mut pos);
                    self.push_stack(BfStackElem::While(loop_vec));
                },
                BfTokens::RLP => {
                    bfpanic!( "MismatchedLoop", "No opening Loop token", src[0], true);
                },
                BfTokens::LTB => {
                    let (action, mut repition) = read_rep(&mut src, &mut pos);
                    while repition > 0 {
                        self.push_stack(BfStackElem::Single(action));
                        repition -= 1;
                    }
                },
                BfTokens::RTB => {
                    bfpanic!( "MismatchedBrackets", "No opening paranthesis", src[0], true);
                },
                _ => self.push_stack(BfStackElem::Single(src[0])),
            }

            src.remove(0);
            pos += 1;

        }

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
fn read_loop(source: &mut Vec<char>, pos: &mut usize) -> Vec<BfWhile> {
    let mut loop_vec: Vec<BfWhile> = Vec::new();

    // Skipping the opening bracket.
    source.remove(0);
    *pos += 1;

    // Loop till we reach the closing bracket.
    loop {
        if source.is_empty() {
            bfpanic!("Unclosed Loop", true);
        }

        match source[0] {
            BfTokens::LLP => {
                // This is the internal loop.
                let nest_loop_vec = read_loop(source, pos);
                loop_vec.push(BfWhile::Nested(Box::new(BfStackElem::While(nest_loop_vec))));
            }
            BfTokens::RLP => {
                loop_vec.push(BfWhile::End);
                return loop_vec;
            }
            BfTokens::LTB => {
                let (action, mut repition) = read_rep(source, pos);
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
                    if source.is_empty() {
                        bfpanic!("Error", "Comment tag not closed", BfTokens::FSL, true);
                    }
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
fn read_rep(source: &mut Vec<char>, pos: &mut usize) -> (char, usize) {
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
        _ => {
            sign = '\0';
            bfpanic!("Syntax Error", "Expected a symbol", source[0], true);
        },
    }

    loop {

        if source.is_empty() {
            bfpanic!("Expected to have a closing paranthesis", true);
        }

        if !source[0].is_digit(10) {

            if source[0].eq(&BfTokens::RTB) {

                if number.is_empty() { return (sign, 1) }

                let Ok(num) = number
                    .trim()
                    .parse::<usize>() else {
                    bfpanic!("SyntaxError", "Cannot parse an empty string", source[0], true);
                    // The return needs to be a return type statement but, since 'bfpanic' doesn't
                    // necessarily mean 'panic' of the program, we need to add an extra dummy panic
                    // to make the program compile.
                    panic!()
                };

                return (sign, num);

            }

            if source[0].ne(&BfTokens::FSL) {
                bfpanic!( "SyntaxError", "Only digits allowed here", source[0], true);
            }

            bfpanic!( "SyntaxError", "Write the comment outside of brackets", source[0], true);

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
