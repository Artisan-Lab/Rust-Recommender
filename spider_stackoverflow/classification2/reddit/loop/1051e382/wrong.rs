fn main()
{
    let mut child: Option<String> = None;

    for i in 0..10 {

        if child.is_some() {
            child.unwrap();
        }

    }
}