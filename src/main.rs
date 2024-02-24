#![allow(unused)]

use std::{f32, io};
use std::cmp::Ordering;
use rand::Rng;
use std::ops::Add;
use std::collections::{HashMap, VecDeque};
use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::num::ParseIntError;

mod restaurant;

// Class 1 - Hello World
fn hello_world() {
    println!("Hello, world!");
}

// Class 2 - Strings
fn string_example() {
    // String is a struct type from Rust, that has a variety of methods
    let mut name: String = String::from("Felipe");
    // &str is the primitive type of string, A &str is made up of two components: a pointer to some bytes,
    // and a length. You can look at these with the as_ptr and len methods:
    let mut surename: &str = "Ramos";
}

// Class 3 - IO
fn io_example() {
    println!("What is your name?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    // Here we are passing a &mut, because we are borrowing a mutable reference from name variable
    // the ownership is still with name variable, but the io library borrows the mutable value, for appending the result
    // into the string
    io::stdin().read_line(&mut name).expect("Didn't receive input");

    println!("Hello {}! {}", name.trim_end(), greeting)
}

// Class 4 - Constants and Shadowing
fn constants_shadow() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = std::f32::consts::PI;
    let age: &str = "47";
    // In Rust, you can define variables that have the same name, but as long they are different types
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL)
}

// Class 5 - Number Data types

fn data_types() {
    // Unsigned int: u8, u16, u32, u64, u128, usize
    // Signed int: i8, i16, i32, i64, i128, isize
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f64: {}", f64::MAX);
}

// Class 6 - If clauses
fn if_clauses() {

    // Normal IF
    let age: i32 = 5;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }

    // Inline IF
    let mut my_age: i32 = 47;
    let can_vote: bool = if my_age > 18 { true } else { false };
    print!("Can vote: {}", can_vote);


    // Using Match
    let age2: i32 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday")
    }
}

// Class 7 - Comparison Operator
fn comp_operator() {
    let my_age: i32 = 18;
    let voting_age: i32 = 18;

    // Here we are sending the voting_age with borrow operator &, because we are just borrowing this value to the function
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Equal => println!("You gained the right to vote"),
        Ordering::Greater => println!("Can vote")
    }
}

// Class 8 - Arrays and Loops
fn basic_arrays() {
    // By definition arrays have fixed length, and in Rust, there is no way to add elements to that,
    // their lengths are fixed at compile time, for enforcing the memory safety.
    // Arrays are allocated in memory as sequential memory addresses, and in some languages, when you are appending to an array,
    // you are basically copying the old array with the new value, to another allocated sequence of the memory.
    // In rust, if you want to increase the elements, you should use Vectors.
    // Vectors in Rust, works in the same way slices works in Golang
    // When you define a Vector, rust compiler makes an extra room in memory allocation, doubling the size of the vector
    // against the first defined size, so you have room for more elements. In the case the number of elements exceeds the
    // allocated memory, Rust will relocate the entire Vector to another addresses, and will double the size again.
    let arr_1: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("1st: {}", arr_1[0]);
    print!("Length: {}", arr_1.len());

    let mut loop_idx = 0;
    loop {
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }
        println!("Val: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }

    let mut loop_index = 0;
    while loop_index < arr_1.len() {
        println!("Element: {}", arr_1[loop_index]);
        loop_index += 1;
    }

    for val in arr_1.iter() {
        println!("Val: {}", val)
    }
}

// Class 9 - Mutable arrays
fn mutable_arrays() {
    // Let's replace all the elements that can be divided by 2, to 0
    // Here we are defining a mutable array, because we want to change some elements
    let mut arr_1: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // For the loop, we will need to can the method iter_mut(), which returns every loop, a &mut(mutable borrow reference)
    for val in arr_1.iter_mut() {
        if (val.abs() % 2 == 0) {
            // for changing its value, we will need to dereference it
            // Remember that this is different from pointers in another language, in Rust we can only borrow a mutable reference to
            // another single variable, in another languages we can create as many pointers we want, here we can only borrow one at a time,
            // like a real object, we can only borrow a car to another person, if we haven't borrowed it to anyone.
            *val = 0;
        }
    }
    println!("Array: {:?}", arr_1)
}

// Class 10 - Tuples
fn basic_tuples() {
    // Tuples are sequential data structures, that can hold different types of data, and it's immutable,
    // off course you can turn a tuple to a mutable tuple, but like arrays, you can't remove/append new elements
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name: {}", my_tuple.1);

    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", my_tuple.0);
}

// Class 11 - Strings Part 2
fn strings_second_example() {
    // Strings are immutable in almost all the languages, and strings are basically an array of bytes
    // on Rust we have the primitive type str and the struct type String
    // The str primitive type is an array of bytes, and it's immutable, there is no push method
    // The String type, if you look inside its implementation, you have a Vector of bytes.


    // Here is a nice example of how string works in many languages, and why they are immutable.
    // We can see here, we have two chinese characters, so, we would have 2 elements on the byte array,
    // but these elements can't be represented with only one byte, that is why this array is of size 6
    // Rust in the background is applying utf-8 encoding, and the reason why strings are immutable, is that
    // imagine if I change the byte on the first position of the byte array, I will be breaking the encoding of the string
    // as the character 世 is represented by more than one byte.
    // You can use the method as_bytes_mut to have a mutable byte array, so you can break the encoding, but this is a unsafe thing
    let mut st1: &str = "世界";
    println!("{:?}", st1.as_bytes());

    let mut st2: String = String::new();
    st2.push('A');
    st2.push_str(" word");
    for word in st2.split_whitespace() {
        println!("{}", word)
    }
    st2.replace("A", "Another");

    // Here we can sum strings
    let st4: String = String::from("1 ");
    let st5: String = String::from("2");
    let st6: String = st4 + &st5;
}

// Class 12 - Casting
fn casting() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
}

// Class 13 - Enums
fn enums() {
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false
            }
        }
    }

    let today: Days = Days::Monday;
    match today {
        Days::Monday => println!("Everybody hates Monday"),
        Days::Tuesday => println!("Donut day"),
        Days::Wednesday => println!("Hump day"),
        Days::Thursday => println!("Pay day"),
        Days::Friday => println!("Almost weekend"),
        Days::Saturday => println!("Weekend"),
        Days::Sunday => println!("Weekend"),
    }
}

// Class 14 - Vectors
// As we talked on Arrays class, vectors are defined on memory as sequential memory addresses,
// and different from arrays, that are fixed in length, here we can assign or remove elements. What Rust does in the
// background, which is done for many different programming languages, is assigning a sequential number of addresses in memory,
// when the size is exceeded, it doubles the current size, and makes a copy of the array to a new sequence.
fn vectors() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = vec![1,2,3,4];
    vec2.push(5);
    println!("1st: {}", vec2[0]);

    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No second value")
    }

    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i)
    }
}

// Class 15 - Functions
// In Rust, you don't need to specify the return statement, the last line of a function will be
// automatically considered the return statement
fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

// Functions can return more than one value
fn return_multiple(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

// As everything is immutable in Rust, if we make a mutable vector, and want to change it on a function
// we would need to send a mutable reference of this guy
fn change_vector(vec1: &mut Vec<i32>) {
    for value in vec1 {
        *value *= 2;
    }
}

fn functions() {
    let mut vec1: Vec<i32> = vec![1, 2, 3];
    change_vector(&mut vec1);
    println!("the sum is: {}", get_sum(5, 5));
    println!("the vector is: {:?}", vec1);
}

// Class 16 - Generics
// For working with Generics, we can't do some operations like sum or subtraction, we will need to use
// a Trait, for being able to execute this operation, in this case the std::ops::Add Trait.
// A trait defines functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify
// that a generic type can be any type that has certain behavior.
// Traits are like interfaces, where you define a method that can be implemented by different Types, or
// by Generics
fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}


// Class 17 - Ownership
// Stack: Stores values in a last in first out format
// Data on stack must have a defined fixed size

// Heap: When putting data on the heap you request a certain amount of space. The OS finds space available
// and returns an address for that space called a pointer.

// RULES
    // 1 - Each value has a variable that is called its owner
    // 2 - There is only one owner at a time
    // 3 - When the owner goes out of the scope, the value disappears, as we don't have garbage collector in Rust
    // that is the way the language free memory, making sure that the application won't allocate more memory than needed
fn ownership() {
    let str1: String = String::from("World");
    let str2: String = str1;

    // If you try to execute the following print statement, it will return an error, because you moved
    // the value from str1 to str2, and now str2 is the owner of the value, so you can't use it.
    // println!("Hello {}", str1)

    // In this case I'm using the & borrow operator, and I can still use vec1, vec2 and vec3, I can
    // create as many borrow references as I want, and use all the variables, remembering that borrow
    // references, you are only allowing the variable which is borrowing the value, to read that value,
    // but if can't modify it.
    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2 = &vec1;
    let vec3 = &vec1;
    println!("{:?}", vec2);
    println!("{:?}", vec3);

    // Mutable references
    // Different from borrow operator(&), we can have mutable borrow operator(&mut), which grants borrower
    // the ability to change the value, this works like a pointer, but different from another languages, since
    // Rust is memory safe, you can only borrow a mutable reference at a time, in the next case if you try to print
    // str4, it will generate an error, because the owner of the mutable reference is currently str5.
    // In another languages that have pointers, like Go and C, you can have as many mutable references as you like,
    // but this is not memory safe. We also have unsafe pointers in Rust, but their use, is not recommended.
    let mut str3: String = String::from("Hello");
    let str4 = &mut str3;
    let str5 = &mut str3;
    // println!("{}", str4)
}


// Class 18 - HashMaps
fn hash_maps() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Flash", "Barry Allen");


    for (k,v) in heroes.iter() {
        println!("{} = {}", k, v)
    }

    if heroes.contains_key("Batman") {
        let the_batman: Option<&&str> = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero")
        }
    }
}

// Class 19 - Structs
// Here, structs can have methods, we just need to call impl <Structname> and define a set of methods
// Structs, can also implement traits, which work like interfaces.
// There is no inheritance in Rust, but like Golang, we have composition, which mean we can have one
// struct, inside the another, and the child struct, can call parent struct methods, or even override parent struct
// methods.
fn structs() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob: Customer = Customer{
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50,
    };
    bob.address = String::from("505 Main St")
}

// Class 20 - Traits
// A trait defines functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify
// that a generic type can be any type that has certain behavior.
fn traits() {
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle { length: f32, width: f32 }
    struct Circle { length: f32, width: f32 }

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Self {
            // remember we don't need to use return statement, and ; for the last statement
            Rectangle{length, width}
        }

        fn area(&self) -> f32 {
            self.length * self.width
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Self {
            Circle{length, width}
        }

        fn area(&self) -> f32 {
            (self.length / 2.0).powf(2.0) * PI
        }
    }
}


// Class 21 - Modules
// Crates: Modules that produce a library or executable
// Modules: Organize and handle privacy
// Packages: Build, test and share crates
// Paths: A way of naming an item such as a struct, function
fn modules() {
    restaurant::order_food();
}

// Class 22 - Error Handling
fn error_handling() {
    let path: &str = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(err ) => panic!("Problem creating file: {:?}", err)
    };

    output.write("Just some\nRandom words".as_ref()).expect("Failed to write to file");
    // write!(output, "Just some\nRandom words").expect("Failed to write to file");
    //
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap())
    }
}

// Class 22 - References part 2

// In Rust we have different types of pointers, they are:
// - Smart pointers
// - Raw pointers
// - References

// We have two types of references, the immutable reference(&), and the mutable reference (&mut)

// In that example, we can se we are using immutable reference(&), which basically borrows a value
// from a variable, and the variable that receives the reference, is borrowing it, so it can only read
// but can't change the value
// you can have as many immutable references as you like
// in the last statement of the function we can se the print doesn't error, but if we do
// let str1: String = String::from("ae");
// let str2: String = str1
// the print will fail, because the ownership moved from str1 to str2
fn immutable_reference() {
    let str1: String = String::from("ae");
    let immut_ref: &String = &str1;
    let immut_ref2: &String = &str1;

    println!("{}", str1)
}

// This is one thing that causes people to go mad about Rust, because the language itself is
// type and memory safe, but how can you can ensure that it has safety, when you have two variables
// sharing a reference?
// There is a rule that changes everything, you can only have one mutable reference to a variable,
// in languages like C or Golang, you can have multiple, in Rust, you can have only one, this helps
// to keep the memory safe characteristic.
// If you do this, you should get an error the last two lines
// let mut str1: String = String::from("test");
// let mut_ref: &mut String = &mut str1;
// let mut_ref_second: &mut String = &mut str1;
// let immut_ref: &String = &str1;
//  println!("{}", mut_ref)
fn mutable_reference() {
    let mut str1: String = String::from("test");
    let mut_ref: &mut String = &mut str1;
    mut_ref.push_str("test 2");

    println!("{}", str1)
}

// For example, this function receives a mutable reference to a vector, with this, you would be able
// to change a vector, changing also the original variable that declared the vector
fn change_my_vector(my_list: &mut Vec<i32>) {
    my_list.push(4);
}

// The rust borrow checker you always check the entire scope where the mutable reference was created,
// so you can release the memory whenever the scope of the function ends, and that is why you can only
// borrow a mutable reference once, because the borrow checker can count a single time, and check when
// the memory can be released, so you can borrow it again to another variable
// Also, take a look in the following code:
//    let mut i:i32 = 1;
//    let ref_i = &mut i;
//    let another_ref_i = ref_i;
// if you try to use ref_i, it will give an error, because the mutable reference can be only borrowed by
// one variable at a time, so basically another_ref_i stole it from ref_i

// Class 23 - Question Marker Operator
// Result is a enum from std, that represents either success or a failure it returns an Ok(T)
// or an Err(E), you can use that on the return of the function to handle errors and valid results
// when you use the question marker ? you are basically evaluating the result, it will assign the T generic type
// to the variable, or it will return an error immediately like an throw on Java language
fn try_to_parse() -> Result<i32, ParseIntError> {
    let x: i32 = "123".parse()?; // x = 123
    let y: i32 = "24a".parse()?; // returns an Err() immediately
    Ok(x + y)                    // Doesn't run.
}


// Class 24 - HTTP Request

fn http_request() -> Result<(), Box<dyn Err>> {
    let resp = reqwest::blocking::get("https://httpbin.org/ip")?.text()?;
    println!("{:#?}", resp);
    Ok(())
}

// ITERATORS

fn iterators() {
    let mut arr_it = [1, 2, 3, 4];
    // when you use .iter(), you are basically borrowing the values like using &
    for val in arr_it.iter() {
        println!("{}", val)
    }

    let mut iter1 = arr_it.iter();
    println!("1st: {:?}", iter1.next());

}


// Class 25 - Closures

fn closures() {
    let can_vote:fn(i32) -> bool = |age: i32| {
        age >= 18
    };
    println!("Can vote: {}", can_vote(8));

    let mut samp1 = 5;
    let print_var = || println!("samp1= {}", samp1);
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);
}

fn closures2() {
    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        func(a,b)
    }
    let sum = |a: i32, b: i32| a + b ;
    let prod = |a: i32, b: i32| a * b ;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));

}

// Class 26 - Smart Pointers

// Stack: Stores values in a last in first out format, and must have a defined fixed size
// Heap: variable size

fn creating_boxes() {
    let b_int1: Box<i32> = Box::new(10);
    println!("b_int1 = {}", b_int1);
}

fn binary_tree_box() {
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }

    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            Self {
                left: None,
                right: None,
                key
            }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self  {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self  {
            self.right = Some(Box::new(node));
            self
        }

        pub fn insert(&mut self, new_value: T) {
            let mut queue: VecDeque<&mut TreeNode<T>> = VecDeque::new();
            queue.push_front(self);

            loop {
                // here we are basically exporting the internal members of the struct
                // this is variable destructured like we have on Javascript
                let TreeNode {
                    ref mut left,
                    ref mut right,
                    ..
                } = queue.pop_back().unwrap();

                match left {
                    Some(node) => {
                        queue.push_front(node);
                    }
                    None => {
                        *left = Some(Box::new(TreeNode::new(new_value)));
                        return;
                    }
                }

                match right {
                    Some(node) => {
                        queue.push_front(node);
                    }
                    None => {
                        *right = Some(Box::new(TreeNode::new(new_value)));
                        return;
                    }
                }
            }
        }
    }

    let mut first_node: TreeNode<i32> = TreeNode::new(1);
    first_node.left = Some(Box::new(TreeNode::new(2)));
    first_node.right = Some(Box::new(TreeNode::new(3)));



    match first_node.right {
        Some(node) => println!("{}", node.key),
        None => println!("there is no value"),
    }
}

fn main() {
    modules();
}