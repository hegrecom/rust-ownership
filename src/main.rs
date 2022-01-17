use std::mem;
use std::ptr;

fn main() {
    ownership_example1();
    partial_move();
    mem_replace();
    swap_impl();
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

    // println!("{:?}", arr); // This is impossible.
    // value borrowed here after move
}

fn partial_move() {
    let person = Person { name: String::from("John"), job: String::from("Engineer") };

    let job = person.job;

    println!("{}", person.name); // This is possible
    // println!("{}", person.job); // This is impossible
}

struct Person {
    name: String,
    job: String
}

fn mem_replace() {
    let mut arr = vec!["Jane".to_string(), "John".to_string(), "Mark".to_string(), "Tom".to_string(), "Kim".to_string()];

    // This is impossible
    // let temp = arr[0];
    // arr[0] = arr[1];
    // arr[1] = temp;

    // This is possible
    let temp1 = mem::take(&mut arr[0]);
    let temp2 = mem::take(&mut arr[1]);
    arr[0] = temp2;
    arr[1] = temp1;

    arr.swap(2, 3);



    println!("{:?}", arr);
}

fn swap_impl() {
    let mut vector_wrapper = VectorWrapper { vector: vec!["A".to_string(), "B".to_string(), "C".to_string()] };

    vector_wrapper.swap(0, 1);

    println!("{:?}", vector_wrapper);
}

#[derive(Debug)]
struct VectorWrapper {
    vector: Vec<String>
}

impl VectorWrapper {
    fn swap(&mut self, a: usize, b: usize) {
        let _ = &self.vector[a];
        let _ = &self.vector[b];

        let ptr = self.vector.as_mut_ptr();

        unsafe { 
            ptr::swap(ptr.add(a), ptr.add(b));
        }
    }
}
