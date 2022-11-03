mod ccm {
    fn my_clone() {
        struct S {
            x: Vec<i32>,
        }
        impl Clone for S {
            fn clone(&self) -> Self {
                S { x: self.x.clone() }
            }
        }
        let mut s1 = S { x: vec![12] };
        let s2 = s1.clone();
        s1.x[0] += 1;
        print!("{} {}", s1.x[0], s2.x[0]);
    }

    fn ref_lifetime() {
        struct TS<'a>(&'a u8);
        enum E<'a, 'b> {
            _A(&'a u8),
            _B,
            _C(bool, &'b f64, char),
            _D(&'static str),
        }
        //More About Lifetimes
        let byte = 34;
        let _ts = TS(&byte);
        let _e = E::_A(&byte);

        fn f<'a>(b: &'a mut u8) -> &'a u8 {
            *b += 1;
            b
        }
        let mut byte = 12u8;
        let byte_ref = f(&mut byte);
        print!("{}", *byte_ref);
    }
}