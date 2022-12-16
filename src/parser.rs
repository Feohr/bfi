mod stack;
mod tape;

use stack::toks::BfTokens;
use stack::BfStack;
use stack::BfStackElem;
use stack::BfWhile;
use tape::BfTape;
use crate::Error;

#[derive(Debug)]
pub struct BfParser {
    stack: BfStack,
}

impl<'a> BfParser {
    // To initialize parsing.
    #[inline]
    pub fn from(src: &'a str) -> Result<Self, Error> {
        Ok(BfParser {
            stack: BfParser::parse(src)?,
        })
    }

    // Iterate over chars in string and push to stack.
    #[inline]
    fn parse(source: &'a str) -> Result<BfStack, Error> {
        let mut stack = BfStack::init_stack();
        stack.stackify(source);
        return Ok(stack);
    }

    // Iterate over the stack.
    pub fn iterate(&mut self) -> Result<(), Error> {
        let mut tape = BfTape::init_tape();
        while !self.stack.item.is_empty() {
            match &self.stack.item[0] {
                BfStackElem::Single(x) => opt_func(&x, &mut tape)?,
                BfStackElem::While(x) => while_loop(&x, &mut tape)?,
            }
            self.stack.pop_stack();
        }
        Ok(())
    }
}

// To process while loop.
fn while_loop(items: &Vec<BfWhile>, tape: &mut BfTape) -> Result<(), Error> {
    'int: loop {
        let context_index = tape.curr_index;
        let mut loop_index = 0;

        loop {
            if tape.cell_container[context_index].value == 0 {
                break 'int;
            }

            match &items[loop_index] {
                BfWhile::Linear(ref single_item) => opt_func(&single_item, tape)?,
                BfWhile::Nested(ref nested_item) => match **nested_item {
                    BfStackElem::Single(ref x) => opt_func(&x, tape)?,
                    BfStackElem::While(ref x) => while_loop(&x, tape)?,
                },
                BfWhile::End => break,
            }

            loop_index += 1;
        }
    }
    Ok(())
}

// To process operations
fn opt_func(ch: &char, tape: &mut BfTape) -> Result<(), Error> {
    match *ch {
        BfTokens::ADD => Ok(tape.inc_curr()?),
        BfTokens::SUB => Ok(tape.dec_curr()?),
        BfTokens::LFT => Ok(tape.move_left()),
        BfTokens::RGT => Ok(tape.move_right()),
        BfTokens::OUT => Ok(tape.output_cell()?),
        BfTokens::INP => Ok(tape.input_cell()?),
        _ => Err(Error::UnknownToken(*ch)),
    }
}
