mod m_traits {
    trait HasSquareRoot {
        fn sq_root(self) -> Self;
    }

    impl HasSquareRoot for f32 {
        fn sq_root(self) -> Self { f32::sqrt(self) }
    }

    impl HasSquareRoot for f64 {
        fn sq_root(self) -> Self { f64::sqrt(self) }
    }

    fn quartic_root<Number>(x: Number) -> Number
        where Number: HasSquareRoot {
        x.sq_root().sq_root()
    }
    print!("{} {}",
           quartic_root(100f64),
           quartic_root(100f32));
}

mod m_traits2 {
    trait HasSquareRoot {
        fn sqrt(self) -> Self;
    }

    impl HasSquareRoot for f32 {
        fn sqrt(self) -> Self { f32::sqrt(self) }
    }

    impl HasSquareRoot for f64 {
        fn sqrt(self) -> Self { f64::sqrt(self) }
    }

    trait HasAbsoluteValue {
        fn abs(self) -> Self;
    }

    impl HasAbsoluteValue for f32 {
        fn abs(self) -> Self { f32::abs(self) }
    }

    impl HasAbsoluteValue for f64 {
        fn abs(self) -> Self { f64::abs(self) }
    }

    fn abs_quartic_root<Number>(x: Number) -> Number
        where Number: HasSquareRoot + HasAbsoluteValue {
        x.abs().sqrt().sqrt()
    }
    print!("{} {}",
           abs_quartic_root(-100f64),
           abs_quartic_root(-100f32));

    fn traits_self() {
        struct Complex {
            re: f64,
            im: f64,
        }
        impl std::fmt::Display for Complex {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{} {} {}i",
                    self.re,
                    if self.im >= 0. { '+' } else { '-' },
                    self.im.abs()
                )
            }
        }
        let c1 = Complex { re: -2.3, im: 0. };
        let c2 = Complex { re: -2.1, im: -5.2 };
        let c3 = Complex { re: -2.2, im: 5.2 };
        print!("{}, {}, {}", c1, c2, c3);

    }
}