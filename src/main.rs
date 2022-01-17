fn main() {
    ownership_example1();
}

fn ownership_example1() {
    let arr = vec![1, 2, 3, 4, 5];

    for element in arr.iter() {
        println!("{}", element);
    }

    println!("{:?}", arr); // This is possible.

    for element in arr.into_iter() {
        println!("{}", element);
    }

    println!("{:?}", arr); // This is impossible.
    // value borrowed here after move
}
