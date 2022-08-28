
mod loops {
    fn loop_one() {
        let a = 10;
        for i in 0..a {
            println!("Num is {}", i)
        }

    }

    fn loop2() {
        for i in 1..10 {
            let b = rand::random();

            println!("b is {}", b);
            let a = if (2 < b) { 3 } else { 2 };
            if (1 < 3) { println!("a is {}", a) } else {return;}
            sleep(Duration::from_secs(3))
        }
    }


}