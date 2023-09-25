use std::io::{self, Result, Write};

trait Stream: Write {}

trait LockableStream<'a>: Stream {
    fn lock(&'a self) -> Box<Stream + 'a>;
}

struct WrapStream<T> {
    w: T,
}

impl<'a> LockableStream<'a> for WrapStream<io::Stdout> {
    fn lock(&'a self) -> Box<Stream + 'a> {
        let locked = WrapStream { w: self.w.lock() };
        Box::new(locked)
    }
}

impl<T: Write> Write for WrapStream<T> {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.w.write(data)
    }

    fn flush(&mut self) -> Result<()> {
        self.w.flush()
    }
}

impl<T: Write> Stream for WrapStream<T> {}

fn main() {
    with_stdio();
    wrapped();
}

fn with_stdio() {
    let mut stream = io::stdout();
    {
        let mut locked = stream.lock();
        writeln!(locked, "Hello!").unwrap();
    }
    writeln!(stream, "Goodbye!").unwrap();
}

fn wrapped() {
    let mut stream = &mut WrapStream { w: io::stdout() } as &mut LockableStream;
    {
        let mut locked = stream.lock();
        writeln!(locked, "Hello!");
    }
    writeln!(stream, "Goodbye!");
}
