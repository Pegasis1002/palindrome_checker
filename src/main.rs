use std::io;

fn main() {
    println!("Please enter a string: \n");
    let mut in_string = String::new();
    io::stdin()
        .read_line(&mut in_string)
        .expect("ERROR: input NOT a string.");

    no_of_char(in_string);
}

fn no_of_char(in_string: String) {
    let x = in_string.len();
    if (x % 2) > 0 {
        println!("'{}' has even number of chars", in_string);
    } else if (x % 2) == 0 {
        println!("'{}' has odd number of chars", in_string);
    }
}
