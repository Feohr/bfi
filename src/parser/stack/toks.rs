#[macro_export]
macro_rules! bfpanic {

    ($err_typ: literal, $msg: literal, $rgt_syn: expr, $level: expr) => {
        match $level {
            true => {
                panic!("\x1b[31;1m{}\x1b[0m: {} Found {:?}", $err_typ, $msg, $rgt_syn);
            },
            false => {
                eprintln!("\x1b[33;1m{}\x1b[0m: {} Found {:?}.", $err_typ, $msg, $rgt_syn);
            },
        }
    };

    ($err_typ: literal, $msg: literal, $level: expr) => {
        match $level {
            true => {
                panic!("\x1b[33;1m{}\x1b[0m: {}", $err_typ, $msg)
            }
            false => {
                eprintln!("\x1b[33;1m{}\x1b[0m: {}.", $err_typ, $msg);
            }
        }
    };

    ($msg: literal, $level: expr) => {
        match $level {
            true => {
                panic!("\x1b[31;1m{}\x1b[0m", $msg);
            }
            false => {
                eprintln!("\x1b[33;1m{}\x1b[0m", $msg);
            }
        }
    };

    () => {
        panic!("\x1b[31;1mSyntax Error!\x1b[31m");
    };

}

// Tokens to use throughout the program.
pub struct BfTokens;
impl BfTokens {
    pub const ADD: char = '+';
    pub const SUB: char = '-';
    pub const LFT: char = '<';
    pub const RGT: char = '>';
    pub const OUT: char = '.';
    pub const INP: char = ',';
    pub const LLP: char = '[';
    pub const RLP: char = ']';
    pub const SPC: char = ' ';
    pub const NXL: char = '\n';
    pub const LTB: char = '(';
    pub const RTB: char = ')';
    pub const TAB: char = '\t';
    pub const FSL: char = '/';
}
