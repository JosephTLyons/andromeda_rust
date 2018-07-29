extern crate rand;

use rand::prelude::*;

fn generate_serial_numbers (number_of_serials: u32, length_of_serial: u32,
                            number: bool, uppercase: bool, lowercase: bool) {
    let mut values_vector: Vec<u8> = create_values_vector (number, uppercase, lowercase);
    let mut vector_of_values_vector: Vec<Vec<u8>> = Vec::new();

    for x in 0..length_of_serial {
        thread_rng().shuffle (&mut values_vector);
        vector_of_values_vector.push (values_vector.clone());
    }

    print_serial_numbers_to_file (vector_of_values_vector);
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

fn print_serial_numbers_to_file (vector_of_values_vector: Vec<Vec<u8>>) {

    let total_possible_combinations: i64 = i64::pow (vector_of_values_vector[0].len() as i64,
                                                     vector_of_values_vector.len() as u32);

    println! ("{} total combinations possible with serial numbers of length {} with {} characters choices",
              total_possible_combinations,
              vector_of_values_vector.len(),
              vector_of_values_vector[0].len());

    //let mut index_vector = vec![0; vector_of_values_vector.len()];

    //print_vector (index_vector);

    // for x in 0..vector_of_values_vector.len() {
    //     for x in 0..vector_of_values_vector[x] {
    //
    //     }
    // }
    // Use length of vector_of_values_vector in for loop to move across vectors and then an inner loop for each vector
}

fn main() {
    generate_serial_numbers (100, 4, true, true, true);
}
