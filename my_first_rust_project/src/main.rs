use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    // let mut sum = 0.0;
    // for i in 0..5 {
    //     let eved_odd = if i % 2 == 0 { "even" } else { "odd" };
    //     println!("{}|{}", i, eved_odd);
    //     sum += i as f64;
    // }
    // println!("sum: {}", sum)

    // let res = sqrt(32.0);
    
    // let i: i32 = 10;
    // let i_ref: &i32 = &i;
    // println!("i ref is {:p}", i_ref);

    // let mut res: f64 = 0.0;
    // modifies(&mut res);
    // println!("{}", res);

    // let x = 2.0 * consts::PI;
    // let abs_difference = (x.cos() - 1.0).abs();
    // println!("{}", abs_difference)


    //Arrays/Slices

    // let arr = [10, 20, 30, 40];
    // let first = arr[0];
    // println!("first: {}", first);

    // for i in 0..4 {
    //     println!("[{}] = {}", i, arr[i]);
    // }

    // println!("length: {}", arr.len());

    // println!("{:?}", arr);

    // let slice = &arr[2..];

    // println!("{:?}", slice);

    // let first = slice.get(0);
    // let last = *slice.get(1).unwrap_or(&-1);

    // println!("first: {} {}", first.is_some(), first.is_none());
    // println!("last: {}", last);
    // println!("first value {}", first.unwrap());

    //Vectors

    // let mut v = Vec::new();
    // v.push(10);
    // v.push(20);

    // let first = v[0];
    // let maybe_first = v.get(0);

    // println!("v is {:?}", v);
    // println!("first is {}", first);
    // println!("maybe_first is {:?}", maybe_first);

    // dump(&v);

    // let arr = [10, 20, 30, 40];
    // for i in arr.iter() {
    //     println!("{}", i);
    // }

    // let sum: i32 = (0..5).sum();
    // println!("sum was {}", sum);

    // let sum: i64 = [10, 20, 30, 40].iter().sum();
    // println!("sum was {}", sum);

    // let slice = &arr;
    // for s in slice.chunks(2)
    // {
    //     println!("window {:?}", s);
    // }
        
    // let mut v1 = vec![10, 20, 30, 40];
    // v1.pop();

    // let mut v2 = Vec::new();
    // v2.push(10);
    // v2.push(20);
    // v2.push(30);

    // assert_eq!(v1, v2);

    // v2.extend(0..2);
    // assert_eq!(v2, &[10, 20, 30, 0, 1]);

    // let mut v1 = vec![1, 10, 8, 8, 4];
    // v1.sort();
    // v1.dedup();
    // println!("{:?}", v1);
    
    // let text = "hello dolly";
    // let mut s = text.to_string();

    // s.push('T');
    // s.push_str("ello");

    // s += "World";

    // s.pop();
    // s.pop();
    
    // dump(text);
    // dump(&s);

    // let arr = array_to_str(&[10, 20, 30, 40]);
    // let res = format!("hello {}", arr);
    // println!("{:?}", res);

    // let first = env::args().nth(1).expect("please supply an argument");
    // let n: i32 = first.parse().expect("not an integer");


    // let str = "test";
    // match str {
    //     _ => {
    //         println!("default");
    //     }
    // }

    // if let Some(idx) = str.find('t') {
    //     println!("Russian hi {}", &str[idx..]);
    // }

    let filename = "C:/Users/paull/Desktop/me/Progammiersprachen/Rust/my_first_rust_project/src/main.rs";
    let mut file = File::open(&filename).expect("can't open the file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    println!("file had {} bytes", text.len());
    

}

// fn sqrt(x: f64) -> f64 {
//     x * x
// }

// fn modifies(x: &mut f64) {
//     *x = 10.0;
// }

// fn dump(arr: &[i32]) {
//     println!("arr is {:?}", arr);
// }

// fn dump(s: &str) {
//     println!("str: {}", s)
// }

// fn array_to_str(arr: &[i32]) -> String {
//     let mut res = '['.to_string();
//     for v in arr {
//         res += &v.to_string();
//         res.push(',');
//     }
//     res.pop();
//     res.push(']');
//     res
// }