fn main() {
    // while_loop();
    for_loop()
}

fn normal_loop() {
    let mut count = 0;

    let data = loop {
        println!("again : {}", count);

        if count == 10 {
            break count;
        }

        count = count + 1;
    };

    println!("{}", data)
}

fn while_loop() {
    let mut c = 0;

    while c < 8 {
        println!("{}", c);
        c = c + 1;
    }
}

fn for_loop() {
    let data = [9; 10];

    for e in data.iter() {
        println!("{}", e)
    }

    for n in 1..8 {
        println!("{}", n)
    }
}
