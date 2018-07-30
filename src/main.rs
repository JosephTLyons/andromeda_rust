extern crate rand;

use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn generate_serial_numbers (number_of_serials: u128, length_of_serial: usize,
                            number: bool, uppercase: bool, lowercase: bool) {
    let mut character_vector: Vec<u8> = create_character_vector (number, uppercase, lowercase);
    let mut vector_of_character_vectors: Vec<Vec<u8>> = Vec::new();
    let max_serial_length: u8 = 20;

    for _ in 0..max_serial_length {
        thread_rng().shuffle (&mut character_vector);
        vector_of_character_vectors.push (character_vector.clone());
    }

    print_serial_numbers_to_file (number_of_serials, length_of_serial, vector_of_character_vectors);
}

fn create_character_vector (number: bool, uppercase: bool, lowercase: bool) -> Vec<u8> {
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

fn print_serial_numbers_to_file (number_of_serials: u128,
                                 length_of_serial: usize,
                                 vector_of_character_vectors: Vec<Vec<u8>>) {

    let total_possible_combinations: u128 = custom_pow (vector_of_character_vectors[0].len() as u128,
                                                        vector_of_character_vectors.len() as u128);

    if total_possible_combinations < number_of_serials {
        println! ("You are requesting {} unique serial numbers, but only {} unique serial numbers can be generated given the inputs; either allow for a larger variety of characters to be used in serial numbers or decrease the amount of serial numbers to be generated.", number_of_serials, total_possible_combinations);
    }

    else {
        let mut serial_file = File::create (number_of_serials.to_string() + "_unique_serials.txt").unwrap();
        let mut single_serial_number_string: String = String::new();
        let mut index_vector: Vec<usize> = vec![0; 20];

        for _ in 0..number_of_serials {
            for y in 0..length_of_serial {
                single_serial_number_string.push (vector_of_character_vectors[y][index_vector[y]] as char);
            }

            // Write single serial number to file
            single_serial_number_string.push_str ("\n");
            serial_file.write (single_serial_number_string.as_bytes());
            single_serial_number_string.clear();

            // Increment index_vector
            increment_index_vector (index_vector.as_mut_slice(), vector_of_character_vectors[0].len());
        }
    }
}

fn increment_index_vector (vec: &mut [usize], upper_rounding_number: usize) {
    // Delete this
            for x in 0..vec.len() {
                print!("{}", vec[x]);
            }

            println!();

    vec[0] += 1;

    for x in 0..vec.len() {
        if vec[x] == (upper_rounding_number) {
            vec[x] = 0;

            if x < vec.len() {
                vec[x + 1] += 1;
            }
        }

        else {
            break;
        }
    }
}

fn custom_pow (base: u128, exponent: u128) -> u128 {
    let mut total: u128 = 1;

    for _ in 0..exponent {
        total *= base;
    }

    return total;
}

fn main() {
    generate_serial_numbers (101, 16, true, false, false);
}
