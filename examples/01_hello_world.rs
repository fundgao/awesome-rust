use std::mem; // Import the `mem` module

struct Person<'a> {
    name: &'a str,
    age: u8
}

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);
    println!("Number of elements in array: {}", xs.len());
    println!("Array occupies {} bytes", mem::size_of_val(&xs));
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}