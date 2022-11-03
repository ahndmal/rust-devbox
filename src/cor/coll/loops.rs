
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

    fn cats_loop() {
        let mut x = [4; 5000];
        x[2000] = 14;
        print!("{}, {} \n", x[1000], x[2000]);

        let mut cats = vec!["Murz", "Lyvko", "Sapko"];
        cats.push("Pukh");
        for cat in cats {
            println!("{}", cat);
        }
    }

    fn guess() {
        let name = "Vasyl";
        println!("Hello {}!", name);
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let length = String::len(&guess);
        println!("length is {}", length);

        let guess: u32 = guess.trim().parse().expect("please type a number");

        println!("You guessed: {}", guess);
    }


}