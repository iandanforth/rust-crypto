// This doesn't work for some reason.
// fn print(foo) {
//     println!("{}", foo);
// }

fn main() {
   
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let tup = (12, 1.4445, -99.5);
    let (x, y, z) = tup;
    println!("{}", y);

    let last = tup.2; // This is how you index into a tuple. Weird!
    println!("{}", last);

    let tup2 = (4, (1, 2, 3), 5);
    let nested = (tup2.1).2; // It doesn't follow the obvious pattern though of tup2.1.2 and you have to start using parens. So ugly.
    // let indval = tup[1]; Fucking bullshit, pick an indexing syntax and stick with it.

    // Arrays
    // These are of fixed size, so only really useful as an enum variant
    let a = [[1], [2], [3], [4], [5]];
    // println!("{}", a.3); // This should work, bs that it doesn't
    // wow you can't index into arrays like you do with tuples.
    let nested2 = a[2][0];
    println!("{}", nested2 );


}
