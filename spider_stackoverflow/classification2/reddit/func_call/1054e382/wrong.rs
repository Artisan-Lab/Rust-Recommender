use std::io::{self, Cursor, prelude::*};

fn file_op<R: Read>(mut reader: R) -> io::Result<()> {
    // Some file operation
    let mut buf = [0u8];
    reader.read(&mut buf)?;
    Ok(())
}

fn foobar<S: Read>(reader: &mut S) -> io::Result<()> {
    file_op(reader)?;
    file_op(reader)
}

fn main() -> io::Result<()> {
    let mut f = Cursor::new(vec![0xffu8]);
    
    foobar(&mut f)?;

    Ok(())
}