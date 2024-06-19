fn main() {
    println!("Hello, world!");
    my_function();
    println!("{}", func2(1, 2))
}

fn my_function() {
    println!("this is a function");
}

fn func2(type1: u32, type2: u32) -> u32 {
    println!("type: {}", type1);
    println!("type: {}", type2);

    // return type1 + type2;
    let t = type1 + type2;
    t
}
