mod queuess {
    fn queue_vec() {
        const SIZE: usize = 40_000;
        let t0 = Instant::now();
        let mut vd = std::collections::VecDeque::<usize>::new();
        for i in 0..SIZE {
            vd.push_back(i);
            vd.push_back(SIZE + i);
            vd.pop_front();
            vd.push_back(SIZE * 2 + i);
            vd.pop_front();
        }
        let t1 = Instant::now();
        while vd.len() > 0 {
            vd.pop_front();
        }
        let t2 = Instant::now();
        print!("{} {}", elapsed_ms(t0, t1), elapsed_ms(t1, t2));
    }

    fn setts() {
        let arr = [6, 8, 2, 8, 4, 9, 6, 1, 8, 0];
        let mut v = Vec::<_>::new();
        let mut hs = std::collections::HashSet::<_>::new();
        let mut bs = std::collections::BTreeSet::<_>::new();
        for i in arr.iter() {
            v.push(i);
            hs.insert(i);
            bs.insert(i);
        }
        print!("Vec:");
        for i in v.iter() { print!(" {}", i); }
        println!(". {:?}", v);
        print!("HashSet :");
        for i in hs.iter() { print!(" {}", i); }
        println!(". {:?}", hs);
        print!("BTreeSet:");
        for i in bs.iter() { print!(" {}", i); }
        println!(". {:?}", bs);
    }
}