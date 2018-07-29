extern crate rand;

use rand::prelude::*;

fn generate_serial_numbers_vector (number_of_serials: u32, length_of_serial: u32,
                                   number: bool, uppercase: bool, lowercase: bool) -> Vec<Vec<u8>> {
    let mut values_vector: Vec<u8> = create_values_vector (number, uppercase, lowercase);
    let mut serial_numbers_vector: Vec<Vec<u8>> = Vec::new();

    for x in 0..length_of_serial {
        thread_rng().shuffle (&mut values_vector);
        serial_numbers_vector.push (values_vector.clone());
    }

    return serial_numbers_vector;
}

// Delete this when done
fn print_vector (vec: Vec<u8>) {
    for x in 0..vec.len() {
        println! ("{}", vec[x] as char);
    }
}

fn create_values_vector (number: bool, uppercase: bool, lowercase: bool) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();

    if number {
        for x in '0' as u8..'9' as u8 + 1 {
            v.push (x);
        }
    }

    if uppercase {
        for x in 'A' as u8..'Z' as u8 + 1 {
            v.push (x);
        }
    }

    if lowercase {
        for x in 'a' as u8..'z' as u8 + 1 {
            v.push (x);
        }
    }

    return v;
}

fn main() {
    generate_serial_numbers_vector (100, 8, true, true, true);
    // Algorithm to cycle through the vectors and print each serial to a file
}
