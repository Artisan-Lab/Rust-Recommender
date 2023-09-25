fn read_csv_file(path: &str) -> Vec<Vec<AnyValue>> {
    let file = File::open(path).expect("could not open file");
    let df = CsvReader::new(file)
        .infer_schema(None)
        .has_header(true)
        .finish()
        .unwrap();
    
    let df_height = df.height();
    // Get all the rows from dataframe
    let mut i = 0;
    let mut rows = Vec::new();
    while i < df_height {
        let row = df.get(i).unwrap();
        rows.push(row.to_owned());
        i += 1;
    }

    return rows;
}