use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;
fn main() {
    // let data: [i32; 3] = [1, 2, 3];

    // let mut dataV: Vec<i32> = Vec::new();

    // let mut dataK = vec![1, 4, 5, 6];

    // dataV.push(10);

    // println!("{:?}", dataV);
    // // println!("{}", &dataK[2]);
    // println!("{}", data[1]);

    // match dataK.get(2) {
    //     Some(val) => println!("value: {}", val),
    //     None => println!("nothing found!"),
    // }

    // for i in &mut dataK {
    //     // println!("{}", i);
    //     *i = *i + 50;
    // }

    // println!("{:?}", dataK);

    // enum SpreedSheet {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreedSheet::Int(3),
    //     SpreedSheet::Text(String::from("Name")),
    //     SpreedSheet::Int(5),
    // ];

    // match &row[1] {
    //     SpreedSheet::Int(i) => println!("{}", i),
    //     _ => println!("not integer"),
    // }

    // string_code();
    hash_map_code();
}

fn string_code() {
    let hello: String = String::from("नमस्ते");

    for i in hello.bytes() {
        println!("{}", i)
    }

    for i in hello.chars() {
        println!("{}", i)
    }
    for g in hello.graphemes(true) {
        println!("tt: {}", g)
    }
}

fn hash_map_code() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for t in text.split_whitespace() {
        // println!("{}", t)
        let count = map.entry(t).or_insert(0);
        *count = *count + 1
    }

    println!("{:?}", map)
}
