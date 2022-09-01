
fn mut_method() {
    struct S { x: u32 }
    impl S {
        fn get_x(&self) -> u32 { self.x }
        fn set_x(&mut self, x: u32) { self.x = x; }
    }
    let mut s = S { x: 12 };
    print!("{} ", s.get_x());
    s.set_x(17);
    print!("{} ", s.get_x());
    
    //

    struct Number {
        x: f64,
    }
    impl Number {
        fn new() -> Number { Number { x: 0. } }
        fn from(x: f64) -> Number { Number { x: x } }
        fn value(&self) -> f64 { self.x }
    }
    let a = Number::new();
    let b = Number::from(2.3);
    print!("{} {}", a.value(), b.value());
}