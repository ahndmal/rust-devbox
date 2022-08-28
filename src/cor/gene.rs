mod gener {

    fn gene_one<T>(ch: char, num1: T, num2: T) -> T {
        if ch == 'a' { num1 } else { num2 }
    }

    fn gene_one_app() {
        let a: i16 = gene_one::<i16>('a', 37, 41);
        let b: f64 = gene_one::<f64>('b', 37.2, 41.1);
        print!("{} {}", a, b);
    }

}
