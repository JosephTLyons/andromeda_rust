extern crate rand;

use andromeda::rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

pub fn interface() {
    println!("Serial number amount: ");
    let number_of_serials: u128 = read!();

    println!("Serial number length (20 or less): ");
    let length_of_serial: usize = read!();

    println!("Enter 'y' to use numbers: ");
    let numbers: bool = 'y' == read!();

    println!("Enter 'y' to use uppercase letters: ");
    let uppercase: bool = 'y' == read!();

    println!("Enter 'y' to use lowercase letters: ");
    let lowercase: bool = 'y' == read!();

    generate_serial_numbers(&number_of_serials, length_of_serial, numbers, uppercase, lowercase);
}

fn generate_serial_numbers(number_of_serials: &u128, length_of_serial: usize, number: bool,
                           uppercase: bool, lowercase: bool) {
    let mut character_vector: Vec<u8> = create_character_vector(number, uppercase, lowercase);
    let mut vector_of_character_vectors: Vec<Vec<u8>> = vec![Vec::new(); length_of_serial];

    // Shuffle character vector and push a copy into vector
    for vec in &mut vector_of_character_vectors {
        thread_rng().shuffle(&mut character_vector);
        *vec = character_vector.clone();
    }

    let total_possible_combinations: u128 = u128::pow(vector_of_character_vectors[0].len() as u128,
                                                      vector_of_character_vectors.len() as u32);

    if can_create_serial_numbers(&number_of_serials, length_of_serial,
                                 &total_possible_combinations) {
        print_serial_numbers_to_file(&number_of_serials, length_of_serial,
                                     &vector_of_character_vectors, &total_possible_combinations);

        print_stats(&number_of_serials, &total_possible_combinations);
    }
}

fn create_character_vector(number: bool, uppercase: bool, lowercase: bool) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();

    if number {
        for x in b'0'..=b'9' {
            vec.push(x);
        }
    }

    if uppercase {
        for x in b'A'..=b'Z' {
            vec.push(x);
        }
    }

    if lowercase {
        for x in b'a'..=b'z' {
            vec.push(x);
        }
    }

    vec
}

fn can_create_serial_numbers(number_of_serials: &u128, length_of_serial: usize,
                             total_possible_combinations: &u128) -> bool {
    if length_of_serial > 20 {
        println!("Serial number length should be no longer than 20 symbols.");

        return false;
    }

    if *total_possible_combinations < *number_of_serials {
        println!("Requested serial number amount: {}", number_of_serials);
        println!("Total possible serial numbers given current inputs: {}", total_possible_combinations);
        println!("Try one or more of the following:");
        println!("- Increasing the length of the serial numbers");
        println!("- Allowing more types of symbols to be used");
        println!("- Decreasing the amount of serial numbers to be generated");

        return false;
    }

    true
}

fn print_serial_numbers_to_file(number_of_serials: &u128, length_of_serial: usize,
                                vector_of_character_vectors: &[Vec<u8>],
                                total_possible_combinations: &u128) {
    let file_name: String = number_of_serials.to_string() + "_unique_serials.txt";
    let mut serial_file = File::create(&file_name).unwrap();
    let mut single_serial_number_string: String = String::new();
    let mut index_vector: Vec<usize> = vec![0; length_of_serial];
    let distance_between_serial_numbers: u128 = total_possible_combinations / number_of_serials;

    for _ in 0..*number_of_serials {
        for y in 0..length_of_serial {
            single_serial_number_string.push(vector_of_character_vectors[y][index_vector[y]] as char);
        }

        // Write single serial number to file
        single_serial_number_string.push_str("\n");
        serial_file.write(single_serial_number_string.as_bytes());
        single_serial_number_string.clear();

        //print_index_vector(&index_vector);

        increase_index_vector_by(&mut index_vector, vector_of_character_vectors[0].len(),
                                 distance_between_serial_numbers);
    }

    println!("Serial numbers printed to file: {}", file_name);
}

// This function is for debugging the program.
#[allow(dead_code)]
fn print_index_vector(index_vector: & [usize]) {
    for index in index_vector {
        print!("{} ", format!("{:02}", index));
    }

    println!();
}

fn increase_index_vector_by(index_vector: &mut [usize], rollover_number: usize,
                            mut distance_between_serial_numbers: u128) {
    let mut increase_value_at_index_x_by: u128;

    for x in (0..index_vector.len()).rev() {
        increase_value_at_index_x_by = distance_between_serial_numbers % rollover_number as u128;

        index_vector[x] += increase_value_at_index_x_by as usize;

        if index_vector[x] >= rollover_number {
            index_vector[x] -= rollover_number;

            if x > 0 {
                index_vector[x - 1] += 1;
            }
        }

        distance_between_serial_numbers /= rollover_number as u128;
    }
}

fn print_stats(number_of_serials: &u128, total_possible_combinations: &u128) {
    println!("Requested serial number amount: {}", *number_of_serials);
    println!("Total possible serial numbers given current inputs: {}", *total_possible_combinations);
    println!("The printed licenses cover {}% of the total license pool",
             (*number_of_serials / *total_possible_combinations) * 100);
}
