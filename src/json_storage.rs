use serde_json::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

pub fn write_to_file(lighting_records: &Vec<Vec<(u8, bool)>>) -> std::io::Result<()> {
    // SO 69449293
    let file = File::create("lighting_records.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, lighting_records)?;
    writer.flush()?;
    Ok(())
}

pub fn read_from_file() -> Result<Vec<Vec<(u8, bool)>>, Error> {
    // will panic if no file
    let file = File::open("lighting_records.json").unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
}
