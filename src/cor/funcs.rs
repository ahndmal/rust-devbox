mod m_functions {

    fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
        (dividend / divisor, dividend % divisor)
    }

    enum E { E1, E2 }

    // struct S { a: i32, b: bool }
    // struct TS (f64, char);
    //
    // fn f1() -> E { E::E2 }
    // fn f2() -> S { S { a: 49, b: true } }
    // fn f3() -> TS { TS (4.7, 'w') }
    // fn f4() -> [i16; 4] { [7, -2, 0, 19] }
    // fn f5() -> Vec<i64> { vec![12000] }
    // print!("{} ", match f1() { E::E1 => 1, _ => -1 });
    // print!("{} ", f2().a);
    // print!("{} ", f3().0);
    // print!("{} ", f4()[0]);
    // print!("{} ", f5()[0]);
    //
    // fn coll_two() -> i32 {
    //     let mut arr:[i32; 7] = [23,31,12,4234,312,31];
    //     let mut leng = arr.len();
    //     arr = [i32; 8];
    //     leng = arr.len();
    //     return leng;
    // }

}