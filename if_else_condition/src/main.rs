fn main() {
    let condition = false;
    let number = 11;

    if number > 0 {
        println!("this is true");
    } else if number < 2 {
        println!("this is number");
    } else {
        println!("this is else statement");
    }

    let number = if condition { 6 } else { 8 };
    println!("{}", number)
}
