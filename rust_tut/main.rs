//https://www.youtube.com/watch?v=ygL_xcavzQ4
//https://github.com/derekbanas/Rust-Tutorial

#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp:: Ordering;


fn main() {
    //Input Function Name in Main, Run|Debug
    //greeting();
    
}

fn greeting() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");

    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn constant() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}

fn integers() {
    // Unsigned integer: u8, u16, u32, u64, u128, usize
    // Signed integer: i8, i16, i32, i64, i128, isize
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max uusize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);
}

fn booleans() {
    let is_true = true;
    let my_grade = 'A';
    // Char is single quote. Strings is double quotes
}


fn math() {
    //Int precision
    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);
    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111);

    //Math Operators
    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 & num_4);
    num_3 +=1;

    //Random Range
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random : {}", random_num);
}


fn conditionals(){
    //IF
    let age: i32 = 8;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50){
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }

    //Ternary Operator
    let mut my_age = 47;
    let can_vote = if my_age >= 18{
        true
    } else{
        false
    };
    println!("Can Vote : {}", can_vote);

    //Match
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not anImportant Birthday"),
    };

    //Match cont. with Ordering
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };
}


fn array() {
    //elements in array must be same datatype, fixed size
    let arr_1 = [1,2,3,4];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    //Loop arrays
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {   //even number
            loop_idx +=1;
            continue;
        }
        if arr_2[loop_idx] == 9{
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx +=1;
    }

    //While Loops
    while loop_idx < arr_2.len() {
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx +=1;
    }

    //For Loops
    for val in arr_2.iter() {
        println!("Val : {}", val);
    }
}



fn tuples() {
    //fixed size, multiple datatypes allowed
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);

}

fn strings() {
    //String is vector of bytes that can be changed. &str is reference that points to string allows viewing
    let mut st1 = String::new();
    st1.push('A');                              //push to end
    st1.push_str(" word");                  //appends to end
    for word in st1.split_whitespace(){      //split on whitepspace
        println!("{}", word);
    }
    //replace in string
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    //vectors-strings
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();      //sort
    v1.dedup();     //remove duplicated chars
    for char in v1 {
        println!("{}",char);
    }
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6]; //non inclusive of the 6th index
    println!("String length : {}", st6.len());
    st5.clear();
    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;       //string 6 will no longer be avilable, st7 will still be avilalb eas used a reference
    for char in st8.bytes(){
        println!("{}", char);
    }
}


fn casting() {
    //cast as specific datatype
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);     //as keyword to cast
}

fn enumerates() {
    //allow custom datatypes
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }
    let today:Day = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut Day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }
    println!("Is today the weekend : {}", today.is_weekend());
}


fn vectors() {
    //Like arrays, can grow if mutable. Can only store values of the same type
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);   //add to vec2
    println!("1st : {}",vec2[0]);
    let second: &i32 = &vec2[1];    //verify values exist
    match vec2.get(1){
        Some(second) => println!("2nd : {}", second),
        None => println!("No second value"),
    }
    for i in &mut vec2 {                //cycle vector and multiply all values by 2
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vector Length {}", vec2.len());   //vector length
    println!("pop : {:?}", vec2.pop());         //pop last value
}



//Functions
//can be placed before or after main()
fn say_hello(){
    println!("Hello");
}
//Inside main() functon
//  say_hello();

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x+y);
}
//  get_sum(x:5, y:4);

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}
//  println!("{}", get_sum_2(5,4));

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y
}
//  println!("{}", get_sum_3(5,4));

fn get_2(x:i32) -> (i32, i32) {
    return (x+1, x+2);
}
//  let(val_1), val2) = get_2(3)
//  println!("Nums : {} {}", val_1, val_2);

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
}
//  let num_list: Vec<i32> = vec![1,2,3,4,5];
//  println!("Sum of list = {}", sum_list(&num_list));


//Generics
use std::ops::Add;

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn generic_types() {
    println!("5 + 4 = {}", get_sum_gen(5,4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2,4.6));

}


//Ownership
//memory managed by system of ownership by rules checked at compile time
//stack - store value last in first out format (LIFO). Data must have defined size
//heap - request certain amount of space - OS will find and return address for space - ref to space is called pointer
//Rules 1) Each value has a variable called its owner.  2)There is only one owner at a time.    3)When owner goes out of scope the value disappears
fn ownership() {
    let str1: String = String::from("World");
    let str2 = str1.clone();     //assignement vs. clone operator to avoid deletion of str1
    //println!("Hello {}", str1);

    //print_str(str1)
    
    //let str3 = print_return_str(str1);
    //println!("str3 = {}", str3);

    //change_string(name: &mut str1);
}

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String{
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String){
    name.push_str(" is happy");
    println!("Message : {}", name);
}


//Hash Maps
use std::collections::HashMap;

fn hash_map() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Flash", "Barry Allen");

    for(k, v) in heroes.iter() {
        println!("{} = {}", k, v);
    }

    println!("Length : {}", heroes.len());

    if heroes.contains_key(&"Batman"){
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero"),
        }
    }
}


//Struct
//custom datatype that holds different types of data
fn structs() {
    const PI: f32 = 3.141592;
    struct Customer{
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer{
        name: String::from("Bob Smith"),
        address: String::from("555 Main St"),
        balance: 234.50
    };
    bob.address = String::from("505 Main St");      //mutable, can change address

    //struct generic
    struct Rectangle<T, U> {
        length: T,
        height: U,
    }
    let rec = Rectangle{length: 4, height: 10.5};

    //traits can be implemented by any stuct with correct trait
    trait Shape{
        fn new(length: f32, width: f32) -> Self;        //constuctor
        fn area(&self) -> f32;
    }

    struct Rectangle2 {length: f32, width: f32};
    struct Circle {length: f32, width: f32};

    impl Shape for Rectangle2{
        fn new(length: f32, width: f32) -> Rectangle2 {
            return Rectangle2{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle{
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rectangle2 = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec Area: {}", rec.area());
    println!("Circ Area: {}", circ.area());


}



//Packages - Crate - Modules
//Crate: Modules that produce a library or executable
//Module: Organize and handle privacy
//Packages: Bui;ld, test and share crates
//Paths: A way of naming an item such as a struct, function
mod restaurant;
use crate::restaurant::order_food;

fn modules() {
    order_food();
}



//Read/Write to Files  -- Error Handling
fn errors() {
    //panic!("Terrible Error");
    let lil_arr = [1,2];
    println!("{}", lil_arr[10]);            //out of index reference will throw an error. Avoid by checking array section
}

//Result has 2 variants: Ok and Err. enum Result<T, E>{ Ok(T), Err(E),}
//T represents the datatype od the value returns and E the type of Error
fn error_handle_files() {
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => { panic!("Problem creating file : {:?}", error);
        }
    };
    write!(output, "Just some\nRandom words").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines(){
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("rand.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?}", error),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };


}


fn iterator() {
    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter(){               //borrows value - doesn't remove collection from memory. Not changable if used this way
        println!("{}", val);
    }
    //arr_it.into_iter()                          //this way consumes collection, but will no longer be able to use collection
    //Make an iterator
    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next());
}


//Closures 
fn closures() {
    //let var_name = |parameters| -> return_type {Body}
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can vote : {}", can_vote(8));

    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);       //5
    print_var();    
    samp1 = 10;
    let mut change_var = || samp1 +=1;
    change_var();
    println!("samp1 = {}", samp1);                                     //11
    samp1 = 10;
    println!("samp1 = {}", samp1);                                     //10

    fn use_func<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }

    let sum = |a, b| a+b;
    let prod = |a, b| a*b;
    println!("5 + 4 = {}", use_func(5,4,sum));
    println!("5 * 4 = {}", use_func(5,4,prod));
}


//Smart Pointers
//address to location in memory (&). String, Vector, Box are types of pointers. Smart have functionality beyond reference to memory
fn smart_pointers() {
    //BOX -  stores data on heap instead of the stack. Stack = LIFO format, defined fixed size (plates stack analogy).
        //Box used when lot of data stored on heap. pass pointers to it on the stack
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);

    //BOX - binary tree example
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self{
            TreeNode {left: None, right: None, key,
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self{
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self{
            self.right = Some(Box::new(node));
            self
        }
    }
    let node1 = TreeNode::new(1).left(TreeNode::new(2)).right(TreeNode::new(3));
}



//Concurrency
//executing different blocks of code independently -- banking example
//concept of threads - parallel programmaing problems are access of data in wrong order 
//or blocked from execution due to confusion over requirements to proceed with execution
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

fn concurrency() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    //to guarantee threads run to completion call join
    thread1.join().unwrap();

    //Bank example
    pub struct Bank {
        balance: f32
    }
    /*
    //Thread can outlive main function which is the bank - will cause errors, fix is smart pointer
    fn withdraw(the_bank: &mut Bank, amt: f32){
        the_bank.balance -= amt;
    }
    let mut bank = Bank{balance: 100.0};
    withdraw(&mut bank, 5.00);
    println!("Balance : {}", bank.balance);

    fn customer(the_bank: &mut Bank) {
        withdraw(the_bank, 5.00);
    }
    thread::spawn(|| {
        customer(&mut bank)
    }).join().unwrap();
    */
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32){
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current Balance : {} Withdrawl a smaller amount", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew : {} . Current Balance", bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }
    
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance: 20.00}));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total {}", bank.lock().unwrap().balance);

}

