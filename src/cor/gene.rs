mod gener {

    fn gene_one<T>(ch: char, num1: T, num2: T) -> T { // Library
        if ch == 'a' { num1 } else { num2 }
    }

    fn gene_app() {
        let a: i16 = gene_one::<i16>('a', 37, 41);
        let b: f64 = gene_one::<f64>('b', 37.2, 41.1);
        let aa: i16 = gene_one('a', 37, 41); // Application
        let bb: f64 = gene_one('b', 37.2, 41.1);
        print!("{} {}", a, b);


        fn f<T>(a: T, _b: T) -> T { a }
        let _a = f(12u8, 13u8);
        let _b = f(12i64, 13i64);
        let _c = f(12i16, 13u16);
        let _d: i32 = f(12i16, 13i16);


        fn gene2<Param1, Param2>(_a: Param1, _b: Param2) {}
        gene2('a', true);
        gene2(12.56, "Hello");
        gene2((3, 'a'), [5, 6, 7]);
    }

    fn structs_gene() {
        struct S<T1, T2> {
            c: char,
            n1: T1,
            n2: T1,
            n3: T2,
        }
        let _s = S { c: 'a', n1: 34, n2: 782, n3: 0.02 };
        struct SE<T1, T2> (char, T1, T1, T2);
        let _se = SE ('a', 34, 782, 0.02);
    }

}
