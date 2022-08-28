
mod collective {
    fn vec_loop() {

        let _a = [""; 0];
        let _aa = vec![true; 0];
        let _b = vec![false; 0];

        let mut ages = vec![1,2,3; 1000];
        for a in ages {
            println!("a is {}", a)
        }
    }
}