use std::io;
use std::collections::HashMap;
mod csv_writer;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Bill{
    // index:String,
    time:String,
    name: String,
    amount: i32
}

pub struct Bills{
    inner: HashMap<String, Bill>
}

impl Bills {
    fn new() -> Self{
        Self{
            inner: HashMap::new()
        }
    }

    fn add(&mut self, bill: Bill){
        self.inner.insert(bill.name.to_string(),bill);
    }

    fn get_all(&self) -> Vec<&Bill>{
        self.inner.values().collect()
    }

    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    fn update(&mut self, name:&str, amount:i32) -> bool{
        match self.inner.get_mut(name){
            Some(bill) =>{
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}


fn get_input() -> Option<String>{
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("please enter your data again")
    }
    let input = buffer.trim().to_owned();
    if &input == ""{
        None
    }else{
        Some(input)
    }
}

fn get_bill_amount() -> Option<i32> {
    println!("Amount");
    loop{
        let input = match get_input() {
            Some(input) => input,
            None => return None
        };

        if &input == "" {
            return None
        }
        let parsed_input: Result<i32,_> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

mod menu {
    use crate::{get_input,get_bill_amount,Bill ,Bills,};
    use chrono::prelude::*;
    use std::{error::Error};
    use std::fs::File;
    use csv::{ReaderBuilder, Writer};

    // pub fn count_rows() -> Result<usize, Box<dyn Error>>{
    //     let csv_path = "E:/newStart/practice/billing_application/test.csv";
    //     let file = File::open(csv_path)?;
        
    //     // Create a CSV reader with flexible settings
    //     let mut csv_reader = ReaderBuilder::new().flexible(true).from_reader(file);
    
    //     // Iterate through CSV records and count the number of rows
    //     let mut row_count = 1;
    //     for _ in csv_reader.records() {
    //         row_count -= 1;
    //     }
    
    //     Ok(row_count)
    // }

    pub fn time() -> String{
        let local: DateTime<Local> = Local::now();
        let times = local.format("%d-%m-%Y %H:%M:%S").to_string();
        return times
    }

    pub fn add_bill(bills : &mut Bills){
        println!("Bills name");
        let name = match get_input(){
            Some(input) => input,
            None => return
        };
        let amount = match get_bill_amount(){
            Some(amount) => amount,
            None => return
        };
        // let index = count_rows().unwrap().to_string();
        let time = time();
        // let bill = Bill{index,time, name, amount};
        let bill = Bill{time, name, amount};
        bills.add(bill);
        println!("bill added");
    }

    pub fn remove_bill(bills: &mut Bills){
        for bill in bills.get_all(){
            println!("{:?}", bill);
        }
        println!("Enter name to remove: ");
        
        let name = match get_input(){
            Some(name) => name,
            None => return
        };
        if bills.remove(&name){
            println!("bill removed");
        }else {
            println!("bill not found")
        }
    }

    pub fn update_bill(bills: &mut Bills){
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
        println!("Enter bill to update");
        let name = match get_input(){
            Some(name) => name,
            None => return
        };

        let amount = match get_bill_amount(){
            Some(amount) => amount,
            None => return
        };

        if bills.update(&name, amount){
            println!("updated")
        }else{
            println!("bill not found")
        }
    }

    pub fn view_bills(bills: &Bills){
        for bill in bills.get_all() {
            println!("{:?}", bill)
        }
    }

    pub fn sort_writer(bills : &Bills)-> Result<(), Box<dyn Error>> {
        let mut bill:Vec<_> = bills.inner.iter().collect();
        // for (key, value) in &bill{
        //     println!("{:?}", value.name)
        // }

        // sort the array
        // bill.sort_by_key(|&(_, ref info)| &info.index);

        // create sorted vector of the hashmap
        let sorted_map:Vec<_> = bill.into_iter().collect();

        // csv path 
        let csv_path = "E:/newStart/practice/billing_application/test.csv";

        let file = File::create(csv_path)?;
        
        let mut writer = Writer::from_writer(file);

        // writer.write_record(&["time", "name" , "amount"])?;

        for (_key, values) in &sorted_map{
            println!("{} {} {}",&values.time, &values.name, &values.amount);
            writer.write_record(&[&values.time.to_string(),&values.name.to_string(),&values.amount.to_string()])?;
        }

        // flush the writer to ensure all the data is written to the file
        writer.flush()?;

        println!("CSV file is written successfully to {}", csv_path);
        Ok(())
    }
}


enum MainMenu{
    AddBill, 
    ViewBill,
    RemoveBill,
    UpdateBill,
    // SortWriter
}

impl MainMenu{
    fn from_string(input: &str) -> Option<MainMenu>{
        match input {
            "1" => Some(MainMenu::AddBill),
            "2" => Some(MainMenu::RemoveBill),
            "3" => Some(MainMenu::ViewBill),
            "4" => Some(MainMenu::UpdateBill),
            // "5" => Some(MainMenu::SortWriter),
            _ => None
        }
    }

    fn show(){
        println!("");
        println!("== Bill Manager ==");
        println!("1. Add Bill");
        println!("2. Remove Bill");
        println!("3. View Bill");
        println!("4. Update Bill");
        println!("5. Sort and Write");
        println!("");
        println!("Enter Selection: ");
    }
}


fn main() {
    // csv_writer::csv_writer();
    // create bill structure 

    let mut bills = Bills::new();
    for i in bills.get_all(){
        println!("{:?}",i)
    }

    loop{
        // display the menu 
        MainMenu::show();
        let input = get_input().expect("no data entered");
        match MainMenu::from_string(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            // Some(MainMenu::SortWriter) => menu::sort_writer(&mut bills).unwrap(),
            None => return 
        }
        menu::sort_writer(&mut bills).unwrap();
        // make a choice, based on input 
    }

}

// todo 
// better way to  set path of the csv file 
// see how to update the file 
// see how to use python with rust 
// make a app 


// start from 92 
