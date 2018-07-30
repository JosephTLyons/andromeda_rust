extern crate rand;

use rand::prelude::*;

fn generate_serial_numbers (number_of_serials: u128, length_of_serial: u64,
                            number: bool, uppercase: bool, lowercase: bool) {
    let mut character_vector: Vec<u8> = create_character_vector (number, uppercase, lowercase);
    let mut vector_of_character_vectors: Vec<Vec<u8>> = Vec::new();

    for x in 0..length_of_serial {
        thread_rng().shuffle (&mut character_vector);
        vector_of_character_vectors.push (character_vector.clone());
    }

    print_serial_numbers_to_file (number_of_serials, vector_of_character_vectors);
}

// Delete this when done
fn print_vector (vec: Vec<u8>) {
    for x in 0..vec.len() {
        println! ("{}", vec[x] as char);
    }
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

fn print_serial_numbers_to_file (number_of_serials: u128, vector_of_character_vectors: Vec<Vec<u8>>) {

    let total_possible_combinations: u128 = custom_pow (vector_of_character_vectors[0].len() as u128,
                                                        vector_of_character_vectors.len() as u128);

    if total_possible_combinations < number_of_serials {
        println! ("You are requesting {} unique serial numbers, but only {} unique serial numbers can be generated given the inputs; either allow for a larger variety of characters to be used in serial numbers or decrease the amount of serial numbers to be generated.", number_of_serials, total_possible_combinations);
    }

    else {
        // This isn't the correct algorithm, algorithm should be able to print all combinations of
        // the serial length, this is just temporary algorithm
        for x in 0..vector_of_character_vectors[0].len() {
            print!("{}) ", x);  // Remove this line after complete

            // Print one serial number
            for y in 0..vector_of_character_vectors.len() {
                print!("{}", vector_of_character_vectors[y][x] as char);
            }

            println!();
        }
    }
}

fn custom_pow (base: u128, exponent: u128) -> u128 {
    let mut total: u128 = 1;

    for x in 0..exponent {
        total *= base;
    }

    return total;
}

fn main() {
    generate_serial_numbers (100, 16, true, true, true);
}
