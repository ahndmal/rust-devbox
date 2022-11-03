mod my_chars {
    fn charring() {
        let e_grave = 'è';
        let japanese_character = 'さ';
        println!("{} {}", e_grave, japanese_character);

        let truthy = 1;
        let falsy = 0;
        print!("{} {} {} {}", truthy != 0, falsy != 0,
               65 as char, 224 as char);

        for i in 0..256 {
            println!("{}: [{}]", i, i as u8 as char);
        }

    }
}