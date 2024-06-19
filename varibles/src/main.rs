fn main() {
    let x = 10;

    println!("value of x {}", x);

    let x = "six";

    println!("value of x {}", x);

    // const COUNT: u32 = 100_000;

    // let aa: u8 = 258;
    // println!("xxx {}", aa);

    let tup = ("value", 120);
    let (values, data) = tup;

    println!("{}", tup.1);
    println!("{}", values);
    println!("{}", data);
    println!("{}", data);

    let arr = [2, 3, 4];
    println!("{:?}", arr);
    println!("{}", arr[1]);

    let ttt = [110; 8];
    println!("{:?}", ttt);
    println!("{:?}", ttt);
    println!("{:?}", ttt);
    println!("{:?}", ttt);
    println!("{:?}", ttt);
}
