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

fn parse_headers(record: &Vec<&str>) -> HeaderLine {
    let mut result_index: Option<usize> = None;
    for (index, field) in record.iter().enumerate() {
        if !field.chars().all(|c| c.is_alphabetic()) {
            return HeaderLine::None { valid: true };
        }
        if *field == "ean" {
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

fn read_stdin() -> Result<(usize, usize), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf)?;
    let fields: Vec<&str> = buf.split(',').collect();
    let header_result = parse_headers(&fields);

    let mut valid_result: usize = 0;
    let mut invalid_result: usize = 0;

    let mut index = 0;
    match header_result {
        HeaderLine::None { valid } => {
            if valid {
                valid_result += 1;
            } else {
                return Ok((0, 0));
            }
        }
        HeaderLine::Some { valid, index: idx } => {
            if valid {
                index = idx;
            } else {
                return Ok((0, 0));
            }
        }
    }

    for line in stdin.lines() {
        let inner_line = line?;
        let fields: Vec<&str> = inner_line.split(',').collect();
        let ean = fields[index];

        if ean.len() > 13 {
            invalid_result += 1;
            continue;
        }
        let padded_ean = format!("{:0<13}", fields[index]);

        if check_digit(&padded_ean) {
            valid_result += 1;
        } else {
            invalid_result += 1;
        }
    }
    Ok((valid_result, invalid_result))
}

fn check_digit(ean: &str) -> bool {
    assert_eq!(ean.len(), 13);
    let coefficients = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 0];
    let mut result: u32 = 0;
    for (ean_char, coef) in ean.chars().zip(coefficients.iter()) {
        let digit = ean_char.to_digit(10).unwrap();
        result += digit * coef;
    }

    result = (10 - (result % 10)) % 10;
    return result == ean.chars().last().unwrap().to_digit(10).unwrap();
}

fn main() {
    if let Err(err) = read_stdin() {
        println!("error running example: {}", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial_ean_test() {
        let ean: String = String::from("0000000000000");
        assert!(check_digit(&ean));
    }

    #[test]
    fn valid_ean() {
        let ean = String::from("4065418448246");
        assert!(check_digit(&ean));
    }
}
