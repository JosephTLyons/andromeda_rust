extern crate rand;

use andromeda::rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

pub fn generate_serial_numbers (number_of_serials: u128, length_of_serial: usize,
                                number: bool, uppercase: bool, lowercase: bool) {
    let mut character_vector: Vec<u8> = create_character_vector (number, uppercase, lowercase);
    let mut vector_of_character_vectors: Vec<Vec<u8>> = vec![Vec::new(); length_of_serial];

    // Shuffle character vector and push a copy into vector
    for x in 0..length_of_serial {
        thread_rng().shuffle (&mut character_vector);
        vector_of_character_vectors[x] = character_vector.clone();
    }

    let total_possible_combinations: u128 = u128::pow(vector_of_character_vectors[0].len() as u128,
                                                      vector_of_character_vectors.len() as u32);

    if can_create_serial_numbers(number_of_serials, length_of_serial, total_possible_combinations) {
        print_serial_numbers_to_file(number_of_serials,
                                     length_of_serial,
                                     vector_of_character_vectors,
                                     total_possible_combinations);
    }
}

fn create_character_vector (number: bool, uppercase: bool, lowercase: bool) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();

    if number {
        for x in '0' as u32..='9' as u32 {
            v.push (x as u8);
        }
    }

    if uppercase {
        for x in 'A' as u32..='Z' as u32 {
            v.push (x as u8);
        }
    }

    if lowercase {
        for x in 'a' as u32..='z' as u32 {
            v.push (x as u8);
        }
    }

    v
}

fn can_create_serial_numbers(number_of_serials: u128, length_of_serial: usize,
                             total_possible_combinations: u128) -> bool {
    if length_of_serial > 20 {
        println!("Serial number length should be no longer than 20 symbols.");

        return false;
    }

    if total_possible_combinations < number_of_serials {
        println!(
            "You are requesting {} unique serial numbers, but only {} unique serial numbers can \
            be generated given the inputs; either allow for a larger variety of characters to be \
            used in serial numbers or decrease the amount of serial numbers to be generated.",
            number_of_serials,
            total_possible_combinations);

        return false;
    }

    true
}

fn print_serial_numbers_to_file (number_of_serials: u128,
                                 length_of_serial: usize,
                                 vector_of_character_vectors: Vec<Vec<u8>>,
                                 total_possible_combinations: u128) {
    let mut serial_file = File::create (number_of_serials.to_string() + "_unique_serials.txt").unwrap();
    let mut single_serial_number_string: String = String::new();
    let mut index_vector: Vec<usize> = vec![0; length_of_serial];
    let distance_between_serial_numbers: u128 = total_possible_combinations / number_of_serials;

    for _ in 0..number_of_serials {
        for y in 0..length_of_serial {
            single_serial_number_string.push (vector_of_character_vectors[y][index_vector[y]] as char);
        }

        // Write single serial number to file
        single_serial_number_string.push_str ("\n");
        serial_file.write (single_serial_number_string.as_bytes());
        single_serial_number_string.clear();

        //print_index_vector(&index_vector);

        increase_index_vector_by(&mut index_vector, vector_of_character_vectors[0].len(),
                                 distance_between_serial_numbers);
    }
}

// This function is for debugging the program.
#[allow(dead_code)]
fn print_index_vector(index_vector: & [usize]) {
    for x in 0..index_vector.len() {
        print!("{} ", format!("{:02}", index_vector[x]));
    }

    println!();
}

fn increase_index_vector_by (index_vector: &mut [usize],
                             upper_rounding_number: usize,
                             mut distance_between_serial_numbers: u128) {
    let mut increase_value_at_index_x_by: u128;

    for x in (0..index_vector.len()).rev() {
        increase_value_at_index_x_by = distance_between_serial_numbers % upper_rounding_number as u128;

        index_vector[x] += increase_value_at_index_x_by as usize;

        if index_vector[x] >= upper_rounding_number {
            index_vector[x] -= upper_rounding_number;

            if x > 0 {
                index_vector[x - 1] += 1;
            }
        }

        distance_between_serial_numbers /= upper_rounding_number as u128;
    }
}
