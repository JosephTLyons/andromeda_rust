extern crate rand;

use rand::prelude::*;

fn generate_serial_numbers (number_of_serials: u64, length_of_serial: u64,
                            number: bool, uppercase: bool, lowercase: bool) {
    let mut character_vector: Vec<u8> = create_values_vector (number, uppercase, lowercase);
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

fn print_serial_numbers_to_file (number_of_serials: u64, vector_of_character_vectors: Vec<Vec<u8>>) {

    let total_possible_combinations: u64 = i64::pow (vector_of_character_vectors[0].len() as i64,
                                                     vector_of_character_vectors.len() as u32) as u64;

    // This isn't the correct algorithm, algorith should be able to print all values, this is just temp

    if total_possible_combinations < number_of_serials {
        println!("You are requesting {} unique licenses, but only {} unique licenses can be generated given the inputs; either allow for a larger variety of characters to be used in serial numbers or decrease the amount of serial numbers to be generated.", number_of_serials, total_possible_combinations);
    }

    else {
        for x in 0..vector_of_character_vectors[0].len() {
            print!("{}) ", x);  // Remove this line after complete

            // Print one serial number
            for y in 0..vector_of_character_vectors.len() {
                print!("{}", vector_of_character_vectors[y][x] as char);
            }

            println!();
        }
    }

    //let mut index_vector = vec![0; vector_of_character_vectors.len()];

    //print_vector (index_vector);

    // for x in 0..vector_of_character_vectors.len() {
    //     for x in 0..vector_of_character_vectors[x] {
    //
    //     }
    // }
    // Use length of vector_of_character_vectors in for loop to move across vectors and then an inner loop for each vector
}

fn main() {
    generate_serial_numbers (1000, 2, true, false, false);
}
