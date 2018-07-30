extern crate rand;

use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn generate_serial_numbers (number_of_serials: u128, length_of_serial: u64,
                            number: bool, uppercase: bool, lowercase: bool) {
    let mut character_vector: Vec<u8> = create_character_vector (number, uppercase, lowercase);
    let mut vector_of_character_vectors: Vec<Vec<u8>> = Vec::new();
    let max_serial_length: u8 = 20;

    for _ in 0..max_serial_length {
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
        let mut serial_file = File::create (number_of_serials.to_string() + "_unique_serials.txt").unwrap();
        let mut single_serial_number_string: String = String::new();
        let mut index_vector: Vec<usize> = vec![0; 20];

        // This isn't the correct algorithm, algorithm should be able to print all combinations of
        // the serial length, this is just temporary algorithm (Delete)
        for x in 0..vector_of_character_vectors[0].len() {
            //print!("{}) ", x);  // Delete this line after complete

            // Create string of one serial number
            for y in 0..vector_of_character_vectors.len() {
                //print!("{}", vector_of_character_vectors[y][x] as char); // Delete
                single_serial_number_string.push (vector_of_character_vectors[y][x] as char);
            }

            // Write single serial number to file
            single_serial_number_string.push_str ("\n");
            serial_file.write (single_serial_number_string.as_bytes());
            single_serial_number_string.clear();
            // println!(); // Detete
        }

        print! ("{}", vector_of_character_vectors[0][index_vector[0]] as char);
        print! ("{}", vector_of_character_vectors[1][index_vector[1]] as char);
        print! ("{}", vector_of_character_vectors[2][index_vector[2]] as char);
        print! ("{}", vector_of_character_vectors[3][index_vector[3]] as char);
        print! ("{}", vector_of_character_vectors[4][index_vector[4]] as char);
        print! ("{}", vector_of_character_vectors[5][index_vector[5]] as char);
        print! ("{}", vector_of_character_vectors[6][index_vector[6]] as char);
        print! ("{}", vector_of_character_vectors[7][index_vector[7]] as char);
        print! ("{}", vector_of_character_vectors[8][index_vector[8]] as char);
        print! ("{}", vector_of_character_vectors[9][index_vector[9]] as char);
        print! ("{}", vector_of_character_vectors[10][index_vector[10]] as char);
        print! ("{}", vector_of_character_vectors[11][index_vector[11]] as char);
        print! ("{}", vector_of_character_vectors[12][index_vector[12]] as char);
        print! ("{}", vector_of_character_vectors[13][index_vector[13]] as char);
        print! ("{}", vector_of_character_vectors[14][index_vector[14]] as char);
        print! ("{}", vector_of_character_vectors[15][index_vector[15]] as char);
        print! ("{}", vector_of_character_vectors[16][index_vector[16]] as char);
        print! ("{}", vector_of_character_vectors[17][index_vector[17]] as char);
        print! ("{}", vector_of_character_vectors[18][index_vector[18]] as char);
        print! ("{}", vector_of_character_vectors[19][index_vector[19]] as char);

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
    generate_serial_numbers (100, 16, false, true, true);
}
