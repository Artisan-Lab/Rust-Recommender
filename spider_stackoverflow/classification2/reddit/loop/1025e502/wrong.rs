pub fn apply_gravity(rows: &mut Vec<Vec<Option<Block>>>) {
    // Iterate backwards, from the bottom row (18th) to the top (0th)
    // Skip the last row as it obviously can't drop any further
    // This way we only need to check if there is empty space directly beneath each block as opposed to anywhere down the column
    for row_index in 18..0 {
        let row_beneath = &rows[row_index + 1];
        let row = &mut rows[row_index];

        for block in row {
            match block {
                &mut None => (),
                &mut Some(ref mut b) => {
                    let column = b.coord.column as usize;
                    match &row_beneath[column] {
                        &None => {
                            drop_block(b);
                            println!("Dropping block");
                        },
                        _ => continue
                    }
                }
            }
        }
    }
}

pub fn drop_block(block: &mut Block) {
    block.coord.row += 1;
}