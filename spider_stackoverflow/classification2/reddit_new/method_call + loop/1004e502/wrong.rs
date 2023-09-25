#[derive(Debug)]
struct Cell {
    line: u8,
    column: u8,
    square: u8,
    value: u8,
}


struct Sudoku {
    cells: Vec<Cell>,
}

impl Sudoku {
    
    fn possible_values(&self, cell: &Cell) -> Vec<u8> {
        [1].to_vec()
    }

    fn solve(&mut self) {
        for cell in self.cells.iter_mut() {
            if cell.value == 0 {
                let possible_values = self.possible_values(&cell);
                match possible_values.len() {
                    0 => panic!("La résolution semble impossible"),
                    1 => {
                        cell.value = possible_values[0];
                        println!("In cell {:?} I would write {}", cell, possible_values[0]);
                    }
                    _ => (),
                }
            }
        }
    }
}
fn main()
{
    
}