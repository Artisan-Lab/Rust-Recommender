fn use_cursor(cursor: &mut io::Cursor<&mut Vec<u8>>) {
    // do some work
}

fn take_reference(data: &mut Vec<u8>) {
    {
        let mut buf = io::Cursor::new(data);

        use_cursor(&mut buf);
    }

    data.len();
}

fn produce_data() {
    let mut data = Vec::new();
    take_reference(&mut data);
    data.len();
}




/*

error[E0382]: use of moved value: `*data`
  --> src/main.rs:14:5
   |
9  |         let mut buf = io::Cursor::new(data);
   |                                       ---- value moved here
...
14 |     data.len();
   |     ^^^^ value used here after move
   |
   = note: move occurs because `data` has type `&mut std::vec::Vec<u8>`, which does not implement the `Copy` trait


*/