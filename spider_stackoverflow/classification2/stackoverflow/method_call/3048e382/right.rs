


fn use_cursor(cursor: &mut io::Cursor<&mut Vec<u8>>) {



    // do some work
}
fn take_reference(data: &mut Vec<u8>) {
    {
        let mut buf = io::Cursor::new(&mut *data);

        use_cursor(&mut buf);
    }

    data.len();
}

fn produce_data() {
    let mut data = Vec::new();
    take_reference(&mut data);
    data.len();
}