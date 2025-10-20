// GTIN structure
// - GS1 company prefix
// - item reference
// - check digit (checksum): last digit
use std::{error::Error, io};

use csv::StringRecord;

#[derive(Debug)]
enum HeaderLine {
    None { valid: bool },
    Some { valid: bool, index: usize },
}

fn parse_headers(record: &StringRecord) -> HeaderLine {
    let mut result_index: Option<usize> = None;
    for (index, field) in record.iter().enumerate() {
        if !field.chars().all(|c| c.is_alphabetic()) {
            return HeaderLine::None { valid: true };
        }
        if field == "ean" {
            if result_index.is_none() {
                result_index = Some(index);
            } else {
                return HeaderLine::Some {
                    valid: false,
                    index: 0,
                };
            }
        }
    }

    if let Some(index) = result_index {
        HeaderLine::Some { valid: true, index }
    } else {
        HeaderLine::Some {
            valid: false,
            index: 0,
        }
    }
}

fn check_ean(ean: &str) -> bool {
    let ean = format!("{:0>13}", ean);

    true
}

fn read_record(record: &StringRecord, index: usize) -> String {
    String::from(record.get(index).unwrap())
}

fn read_stdin() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());

    println!("headers: {:?}", rdr.headers()?);
    println!("header parsed: {:?}", parse_headers(rdr.headers()?));

    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        // println!("{:?}", record);
        println!("ean: {}", read_record(&record, index));
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_stdin() {
        println!("error running example: {}", err);
    }
}
