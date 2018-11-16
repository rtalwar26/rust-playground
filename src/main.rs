use std::collections::HashMap;
use std::rc::Rc;
mod arith;
mod models;
fn main() {
    println!("Hello, world! {:?}", build_vector());
    traverse_u8();
    traverse_i8();
    print_ascii(b'A'); // b'A' is same as 65u8
    print_ascii(b' ');
    print_maths();

    let v: Vec<f64> = vec![1.2, 3., 4., 5.5];
    print_slice(&v);

    println!("printing array slices");
    let a: [f64; 4] = [1.2, 3., 4., 5.5];

    print_slice(&a[2..]);

    move_out_of();

    move_struct();

    test_rc();

    let mut t = get_hash_map();

    show(&mut t);

    println!("table is {:?}", t);

    println!("result from mod is {}",arith::add(2,3));
//    println!("result from mod is {}",arith::extra::mult(2,3));
   let d = models::dog();
//    println!("dog name {}",d.species);

}

fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    return v;
}

fn traverse_u8() {
    let mut i: u8 = 3;
    while i != 1 {
        println!("u8 is now {}", i);
        i = i.wrapping_add(1);
    }
}

fn traverse_i8() {
    let mut i: i8 = 1;
    while i != 0i8 {
        println!("i8 is now {}", i);

        i = i.wrapping_add(1);
    }
}

fn print_ascii(i: u8) {
    println!("ascii code is {}", i);
}

fn print_maths() {
    println!("sqrt {}", (4f64).sqrt());
}

fn print_slice(s: &[f64]) {
    for i in s {
        println!("slice value {}", i);
    }
}

fn move_out_of() {
    let mut v: Vec<String> = Vec::with_capacity(4usize);

    for i in 101..106 {
        v.push(i.to_string());
    }

    let second = &v[1];
    println!("{}", second);
}

fn move_struct() {
    let l = Label { number: 3 };
    print_struct_name(l);
    println!("number after move is {}", l.number);
}

#[derive(Copy, Clone)]
struct Label {
    number: u32,
}

fn print_struct_name(l: Label) {
    println!("number is {}", l.number);
}

fn test_rc() {
    let s: Rc<String> = Rc::new("hello".to_string());
    let t: Rc<String> = s.clone();
    let u = s.clone();
}

type Table = HashMap<String, Vec<String>>;

fn get_hash_map() -> Table {
    let mut table = Table::new();

    table.insert("key1".to_string(), vec!["".to_string(), "".to_string()]);

    return table;
}

fn show(t: &mut Table) {
    for (artist, works) in t {
        println!("work by {}", artist);
        works.sort();
        for work in works {
            println!("is {}", work);
        }
    }
}

/*enum BinaryTree<T> {
    None,
    left(Box<BinaryTree<T>>),
    right(Box<BinaryTree<T>>),
    value(T),
}

impl<T> BinaryTree<T> {
    fn add(&self, value: T)->&Self {
        match self {
            BinaryTree::None => {
                return &BinaryTree::value(value);
            },
            BinaryTree::value(v)=>{
                return self;
            },
            BinaryTree::left(t) =>{
                return self;
            },
            BinaryTree::right(t) =>{
                self
            }
        }
    }
}
*/