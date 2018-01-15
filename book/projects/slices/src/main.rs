fn main() {
    // Slices are another data type that does not have ownership
    // 

    let phrase = String::from("Hello, world!");

    let ind = first_word(&phrase);

    println!("{:?}", ind );
}

fn first_word(s: &String) -> usize {

    // To iterate over a string you need to turn it into an array of bytes
    let bytes = s.as_bytes();

    // Here we destructure the tuples returned by enumerate() and assign them to variables
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // This is 'byte literal' syntax and is looking for a space
            return i;
        }
    }

    return s.len();
} 
