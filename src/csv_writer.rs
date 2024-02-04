use std::error::Error;
use std::fs::File;
use csv::Writer;
use std::collections::HashMap;

#[derive(Debug)]
pub struct info {
    index:i32,
    name: String,
    age: i32
}

pub struct Hashes {
    inner: HashMap<String, info> 
}


impl Hashes {
    
    fn new() -> Self{
        Self{
            inner:HashMap::new()
        }
    }

    fn get_all(&self) -> Vec<&info>{
        self.inner.values().collect()
    }
    
}

pub fn csv_writer() -> Result<(), Box<dyn Error>> {
    // Create a HashMap with some data
    let mut my_map = Hashes::new();
    my_map.inner.insert("key1".to_owned(), info{index:1,name:"n".to_owned(), age:1});
    my_map.inner.insert("key2".to_owned(), info{index:2,name:"a".to_owned(), age:2});
    my_map.inner.insert("key3".to_owned(), info{index:3,name:"h".to_owned(), age:3});
    my_map.inner.insert("key4".to_owned(), info{index:4,name:"i".to_owned(), age:4});


// Convert the HashMap to a vector of key-value pairs
    let mut sorted_vec: Vec<_> = my_map.inner.into_iter().collect();

    // Sort the vector by the info.index field
    sorted_vec.sort_by_key(|&(_, ref info)| info.index);

    // Create a new HashMap from the sorted vector
    let sorted_map: Vec<_> = sorted_vec.into_iter().collect();

    // Specify the path for the CSV file
    let csv_path = "E:/newStart/test_file/test_file/test.csv";

    // Open the CSV file for writing
    let file = File::create(csv_path)?;

    // Create a CSV writer
    let mut writer = Writer::from_writer(file);

    // Write the header row
    writer.write_record(&["Index" ,"Name", "Age"])?;

    // Iterate through the vector and write each struct to the CSV file
    for (key, value) in &sorted_map {
        println!("{:?} {:?} {:?}", value.index, value.name ,value.age);
        writer.write_record(&[&value.index.to_string(), &value.name, &value.age.to_string()])?;
    }

    // Flush the writer to ensure all data is written to the file
    writer.flush()?;

    println!("CSV file written successfully to: {}", csv_path);

    Ok(())
}

fn main() {
    csv_writer();
}
