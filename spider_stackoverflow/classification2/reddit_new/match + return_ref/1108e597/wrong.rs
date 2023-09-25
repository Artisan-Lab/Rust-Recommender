fn read_document(id : &str) -> Option<&Document>{
    match DOCUMENTS.read(){
        Err(_) => {
            eprintln!("Document store is poisoned.");
            None},
        Ok(documents) => {
            match documents.get(id) {
                Some(document) => Some(document),
                _ => None
            }
        }
    }
}