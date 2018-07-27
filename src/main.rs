fn generate_serial_numbers (number_of_serials: u32, length_of_serial: u32) {
    let mut alphabet_vector: Vec<u8> = create_alphabet_vector (false, false, true);

    // for x in 0..number_of_serials {
    //     serial_vector.push (create_alphabet_vector());
    // }

    for x in &alphabet_vector {
        println! ("{}", *x as char);
    }
}

fn create_alphabet_vector (number: bool, uppercase: bool, lowercase: bool) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();

    if (number) {
        for x in '0' as u8..'9' as u8 + 1 {
            v.push (x);
        }
    }

    if (uppercase) {
        for x in 'A' as u8..'Z' as u8 + 1 {
            v.push (x);
        }
    }

    if (lowercase) {
        for x in 'a' as u8..'z' as u8 + 1 {
            v.push (x);
        }
    }


    return v;
}

fn main() {
    generate_serial_numbers (100, 8);
}
