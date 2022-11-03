mod collective {
    fn vec_loop() {
        let _a = [""; 0];
        let _aa = vec![true; 0];
        let _b = vec![false; 0];

        let mut ages = vec![1,2,3; 1000];
        for a in ages {
            println!("a is {}", a)
        }

        let _array1: [char; 3] = ['x', 'y', 'z'];
        let _array2: [f32; 200] = [0f32; 200];
        let _vector1: Vec<char> = vec!['x', 'y', 'z'];
        let _vector2: Vec<i32> = vec![0; 5000];
    }

    fn desc_coll() {
        let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
        use std::cmp::Ordering;
        fn desc(a: &i32, b: &i32) -> Ordering {
            if a < b { Ordering::Greater } else if a > b { Ordering::Less } else { Ordering::Equal }
        }
        arr.sort_by(desc);
        print!("{:?}", arr);
    }

    fn sort_clos() {
        let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
        use std::cmp::Ordering;
        arr.sort_by(|a, b|
            if a < b { Ordering::Greater } else if a > b { Ordering::Less } else { Ordering::Equal });
        print!("{:?}", arr);
    }
}