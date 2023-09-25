use csv::{Reader, StringRecord};

fn read_csv_from_file_and_print(file_path: &str) -> Result<Vec<StringRecord>, anyhow::Error> {
    let mut file_reader = Reader::from_path(file_path)?;
    let mut record_rows = Vec::new();

    for result in file_reader.records() {
        let record = result?;
        record_rows.push(record)
    }

    Ok(record_rows)
}

fn main() -> Result<(), anyhow::Error> {
    let file_data = read_csv_from_file_and_print("./customers.csv")?;

    println!("File below");
    println!("{:?}", file_data);

    Ok(())
}
