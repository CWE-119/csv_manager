use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use csv::Writer;


pub fn csv_writer() -> Result<(), Box<dyn Error>> {
    // Create a HashMap with some data
    let mut my_map = HashMap::new();
    my_map.insert("key1", "value1");
    my_map.insert("key2", "value2");
    my_map.insert("key3", "value3");
    my_map.insert("key4","value5");

    // Specify the path for the CSV file
    let csv_path = "E:/newStart/practice/billing_application/test.csv";

    // Open the CSV file for writing
    let file = File::create(csv_path)?;

    // Create a CSV writer
    let mut writer = Writer::from_writer(file);

    // Write the header row if needed
    // writer.write_record(&["Key", "Value"])?;

    // Iterate through the HashMap and write each key-value pair to the CSV file
    for (key, value) in my_map {
        writer.write_record(&[key, value])?;
    }

    // Flush the writer to ensure all data is written to the file
    writer.flush()?;

    println!("CSV file written successfully to: {}", csv_path);

    Ok(())
}
