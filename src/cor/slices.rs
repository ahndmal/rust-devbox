mod slices_m {
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
        let _r1 = 4..4; let _a1 = &arr[_r1];
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