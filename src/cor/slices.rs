mod slices_m {
    fn my_collect() {
        let arr = [36, 1, 15, 9, 4];
        let v = arr.iter().collect::<Vec<&i32>>();
        print!("{:?}", v);
    }

    fn slice_filters() {
        let arr = [66, -8, 43, 19, 0, -31];
        for n in arr.iter().filter(|x| **x < 0) {
            print!("{} ", n);
        }

        // map

        let arr = [66, -8, 43, 19, 0, -31];
        for n in arr.iter().map(|x| *x * 2) {
            print!("{} ", n);
        }
        // enumerate
        let arr = ['a', 'b', 'c'];
        for (i, ch) in arr.iter().enumerate() {
            print!("{} {}, ", i, *ch);
        }
        // summ

        print!("{}", [45, 8, -2, 6].iter().sum::<i32>());
    }

    fn min_max() {
        let arr = [45, 8, -2, 6];
        match arr.iter().min() {
            Some(n) => print!("{} ", n),
            _ => (),
        }
        match arr.iter().max() {
            Some(n) => print!("{} ", n),
            _ => (),
        }
    }

    fn slice_muta2() {
        let slice1 = &[3, 4, 5];
        let slice2 = &[7, 8];
        let mut iterator = slice1.iter();
        for item_ref in iterator {
            print!("[{}] ", *item_ref);
        }
        iterator = slice2.iter();
        for item_ref in iterator {
            print!("({}) ", *item_ref);
        }
        //

        for item_ref in (&mut [11u8, 22, 33]).iter_mut() {
            *item_ref += 1;
            print!("{} ", *item_ref);
        }
        for item_ref in [44, 55, 66].iter_mut() {
            *item_ref += 1;
            print!("{} ", *item_ref);
        }
        for item_ref in vec!['a', 'b', 'c'].iter_mut() {
            *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
        }
        //
        let mut vec: Vec<char> = vec!['a', 'b', 'c'];
        let vec_it: std::slice::IterMut<char> = vec.iter_mut();
        for item_ref in vec_it {
            *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
            print!("{} ", *item_ref);
        }
        /////
        let slice = &mut [11u8, 22, 33];
        for item_ref in slice.iter_mut() {
            *item_ref += 1;
        }
        print!("{:?} ", slice);
        let mut arr = [44, 55, 66];
        for item_ref in arr.iter_mut() {
            *item_ref += 1;
        }
        print!("{:?} ", arr);
        let mut vec = vec!['a', 'b', 'c'];
        for item_ref in vec.iter_mut() {
            *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
        }
        print!("{:?} ", vec);
    }

    fn slicing() {
        fn min(arr: &[i32]) -> i32 {
            // Let's assume 'arr' is not empty.
            let mut minimum = arr[0];
            for i in 1..arr.len() {
                if arr[i] < minimum { minimum = arr[i]; }
            }
            minimum
        }
        print!("{} ", min(&[23, 17]));
        print!("{}", min(&vec![55, 22, 33, 44]));
    }

    fn main_slices() {
        fn min(arr: &[i32]) -> i32 {
            // Let's assume 'arr' is not empty.
            let mut minimum = arr[0];
            for i in 1..arr.len() {
                if arr[i] < minimum { minimum = arr[i]; }
            }
            minimum
        }
        let arr = [23, 17, 12, 16, 15, 2];
        let range = 2..5;
        let slice_ref = &arr[range];
        print!("{}", min(slice_ref));
    }

    fn slice3() {
        let arr = [55, 22, 33, 44, 66, 7, 8];
        let v = vec![55, 22, 33, 44, 66, 7, 8];
        let sr1 = &arr[2..5];
        let sr2 = &v[2..5];
        print!("{:?} {:?} {:?} {:?}", sr1, sr2, &sr1[1..2], &sr1[1]);
    }

    //Out-of-Range Slicing
    fn out_range_s() {
        let arr = [55, 22, 33, 44, 66];
        let _r1 = 4..4;
        let _a1 = &arr[_r1];
        let _r2 = 4..3; //let _a2 = &arr[_r2];
        let _r3 = -3i32..2; //let _a3 = &arr[_r3];
        let _r4 = 3..8; //let _a4 = &arr[_r4];
    }

    //Open-Ended Ranges and Slicing
    fn open_end_slice() {
        let arr = [11, 22, 33, 44];
        let n = 2;
        let sr1 = &arr[0..n];
        let sr2 = &arr[n..arr.len()];
        print!("{:?} {:?}", sr1, sr2);
        //[11, 22] [33, 44]
    }

    fn iter_muta() {
        let mut r = "abc".chars();
        for i in r {
            r = "XY".chars();
            print!("{} {}; ", i, r.next().unwrap());
        }
    }
}