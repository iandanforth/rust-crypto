// Generics are abstract stand-ins for concrete types or other properties

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item
        }
    }

    return largest
}

// Generic version of the above
fn generic_largest<T>(list: %[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item
        }
    }

    return largest
}

fn main() {
    println!("Hello, world!");

    let num_list = vec![20, 30, 400, 50, 60, 99];

    let mut largest = num_list[num_list.len()-1];

    for number in num_list {
        if number > largest {
            largest = number;
        }
    }

    println!("{:?}", largest);


}
