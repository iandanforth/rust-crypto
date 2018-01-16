fn main() {
    // Slices are another data type that does not have ownership
    // 

    let phrase = String::from("Hello,world!");

    let ind = first_word(&phrase);

    println!("{:?}", ind );

    // Slicing syntax is the same as python other than using .. rather than :
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = a[0..3];
}

fn first_word(s: &str) -> &str {

    // To iterate over a string you need to turn it into an array of bytes
    let bytes = s.as_bytes();

    // Here we destructure the tuples returned by enumerate() and assign them to variables
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // This is 'byte literal' syntax and is looking for a space
            return &s[..i];
        }
    }

    return &s;
} 
