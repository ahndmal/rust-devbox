mod tuplee {
    fn some_tupling() {
        let a: () = ();
        let b = { 12; 87; 283 };
        let c = { 12; 87; 283; };
        let d = {};
        let e = if false { };
        let f = while false { };
        print!("{:?} {:?} {:?} {:?} {:?} {:?}",
            a, b, c, d, e, f);
    }
}