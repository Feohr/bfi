use crate::Error;

const TAPE_SIZE: usize = 255_usize;
const CELL_SIZE: u8 = 255_u8;

#[derive(Debug)]
pub struct BfCell {
    pub value: u8,
}

impl BfCell {
    // to create an empty cell.
    #[inline]
    fn create_cell() -> Self {
        BfCell {
            value: 0_u8,
        }
    }

    // To increment cell.
    fn inc_cell(&mut self) {
        if self.value < CELL_SIZE {
            return self.value += 1_u8;
        }
        self.value = 0_u8;
    }

    // To decrement cell.
    fn dec_cell(&mut self) {
        if self.value > 0_u8 {
            return self.value -= 1_u8;
        }
        self.value = 255_u8;
    }

    // To get the cell output.
    fn as_char(&self) -> char {
        return self.value as char;
    }

    // To put the input inside the cell.
    fn put_cell(&mut self, input: u8) {
        self.value = input;
    }
}

#[derive(Debug)]
pub struct BfTape {
    pub cell_container: Vec<BfCell>,
    pub curr_index: usize,
    index_counter: usize,
}

impl BfTape {
    // to intitialize the tape.
    #[inline]
    pub fn init_tape() -> Self {
        BfTape {
            cell_container: vec![BfCell::create_cell()],
            curr_index: usize::MIN,
            index_counter: usize::MIN,
        }
    }

    // To fetch current value from the tape.
    fn get_container(&self) -> Result<&BfCell, Error> {
        let Some(cell) = self.cell_container.get(self.curr_index) else {
            return Err(Error::OutOfTapeIndex(self.curr_index));
        };
        Ok(cell)
    }

    // To fetch mutable current value from the tape.
    fn get_container_mut(&mut self) -> Result<&mut BfCell, Error> {
        let Some(cell) = self.cell_container.get_mut(self.curr_index) else {
            return Err(Error::OutOfTapeIndex(self.curr_index));
        };
        Ok(cell)
    }

    // To add a new cell to the tape.
    pub fn add_cell(&mut self) {
        if self.cell_container.len() < TAPE_SIZE {
            self.cell_container.push(BfCell::create_cell());
            self.index_counter += 1_usize;
        }
        self.curr_index = self.cell_container.len() - 1_usize;
    }

    // To print the cell value to stdout.
    pub fn output_cell(&self) -> Result<(), Error> {
        Ok(
            print!("{}", self.get_container()?.as_char())
        )
    }

    // To get input from stdin and store in cell.
    pub fn input_cell(&mut self) -> Result<(), Error> {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        let input: u32 = buf.trim().parse::<u32>()?;
        self.get_container_mut()?.put_cell(input as u8);
        Ok(())
    }

    // Moving to the left cell.
    pub fn move_left(&mut self) {
        if self.curr_index > 0_usize {
            return self.curr_index -= 1_usize;
        }
        self.add_cell();
    }

    // Moving to the right cell.
    pub fn move_right(&mut self) {
        match self.curr_index {
            curr_index if curr_index >= TAPE_SIZE => self.curr_index = 0_usize,
            curr_index if curr_index == self.index_counter => self.add_cell(),
            _ => self.curr_index += 1_usize,
        }
    }

    // Incrementing Current cell.
    pub fn inc_curr(&mut self) -> Result<(), Error> {
        Ok(self.get_container_mut()?.inc_cell())
    }

    // Decrementing current cell.
    pub fn dec_curr(&mut self) -> Result<(), Error> {
        Ok(self.get_container_mut()?.dec_cell())
    }
}
