fn main() {
    // let x = 5;
    // let y = x;

    // println!("{}, {}", x, y);

    let str = String::from("Hello World");
    let s2 = " Rust";

    // let data = str.clone();

    // println!("{} : {}", str, data);
    // cal_len_main()

    // println!("{}", first_word(s2));

    let arr = [1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", &arr[..3]);
    println!("{:?}", &arr[2..]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        println!("{}: {}", i, item);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}

fn cal_len(s: &mut String) -> usize {
    s.push_str("new data");
    let l = s.len();

    l
}

fn cal_len_main() {
    let mut s1 = String::from("string value");
    let l = cal_len(&mut s1);

    println!("s1: {} {}", s1, l)
}
