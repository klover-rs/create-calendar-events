use chrono::prelude::*;
use std::io;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use std::process::Command;




fn main() {
    println!("enter your title:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    let end_input = input.trim();
    println!("your title: {}", end_input);
    println!("SUMMARY:{}", end_input);

    println!("enter your description:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("failed to read inpuit");
    let end_input1 = input1.trim();
    println!("your description: {}", end_input);
    println!("DESCRIPTION:{}", end_input);
    let local_time = Local::now();
    

    // Convert the local time to a timestamp format
    let timestamp = local_time.format("%Y%m%dT%H%M%S").to_string();
    
    println!("Timestamp: {}", timestamp);
    println!("{}", local_time);
    println!("Enter the end date (YYYY-MM-DD):");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let end_date = input2.trim();

    let parsed_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d").expect("failed to parse date");
    let end_datetime = Local.from_local_date(&parsed_date).unwrap().and_hms(23, 59, 59);

    let timestamp2 = end_datetime.format("%Y%m%dT%H%M%S").to_string();

    println!("{}", timestamp2);

    let mut file = File::create("example.ics").expect("failed to create file.");

    

    let uuid = Uuid::new_v4();


    println!("Generate UID: {}", uuid);

    let vcalendar = format!(
        r#"BEGIN:VCALENDAR
BEGIN:VEVENT
UID:{}
SEQUENCE:0
DTSTAMP:{}
DTSTART:{}
DTEND:{}
SUMMARY:{}
DESCRIPTION:{}
END:VEVENT
END:VCALENDAR"#,
        uuid, timestamp, timestamp, timestamp2, end_input, end_input1
    );
    
    println!("{}", vcalendar);
    file.write_all(vcalendar.as_bytes())
        .expect("Failed to write to file");

        Command::new("open")
        .arg("example.ics")
        .spawn()
        .expect("Failed to open the file");

    
}
