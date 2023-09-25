#[derive(Debug)]
struct E {
    v: Vec<u8>,
}

impl Error for E {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for E {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")
    }
}

fn main() -> Result<(), E> {
    let v = vec![0; 1];

    Err(E { v })?;
    Err(E { v })?;

    Ok(())
}