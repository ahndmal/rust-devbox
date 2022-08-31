mod strs {

    fn change_string() {
        let mut name: str = "Murz";
        let s1 = String::new();
        let s2 = String::from("");
        let s3 = "".to_string();
        let s4 = "".to_owned();
        let s5 = format!("");
        print!("({}{}{}{}{})", s1, s2, s3, s4, s5);

        //
        let mut dyn_str = "Hello".to_string();
        dyn_str.push_str(", ");
        dyn_str.push_str("world");
        dyn_str.push_str("!");
        print!("{}", dyn_str);

        let word = "bye".to_string();
        let w1: &str = &word;
        let w2: &String = &word;
        print!("{} {}", w1, w2);
    }
}