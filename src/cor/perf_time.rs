mod perf {
    use std::time::Instant;

    fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
        let t = t2 - t1;
        t.as_secs() as f64 * 1000.
            + t.subsec_nanos() as f64 / 1e6
    }
    fn exe() {
        let time0 = Instant::now();
        for i in 0..10_000 {
            println ! ("{}", i);
        }
        let time1 = Instant::now();
        println!("{}", elapsed_ms(time0, time1));
    }
}