mod nummers {
    fn nums_one() {
        //
        let a2= 1e5;
        let a3: i8 = 600;

        let _: i8 = 127;
        let _: i16 = 32_767;
        let _: i32 = 2_147_483_647;
        let _: i64 = 9_223_372_036_854_775_807;
        let _: isize = 100; // The maximum value depends on the target architecture
        let _: u8 = 255;
        let _: u16 = 65_535;
        let _: u32 = 4_294_967_295;
        let _: u64 = 18_446_744_073_709_551_615;
        let _: usize = 100; // The maximum value depends on the target architecture
        let _: f32 = 1e38;
        let _: f64 = 1e308;

        let one_thousand = 1e3;
        let one_million = 1e6;
        let thirteen_billions_and_half = 13.5e9;
        let twelve_millionths = 12e-6;

        let hexadecimal = 0x10;
        let octal = 0o10;
        let binary = 0b10;
        let mut n = 10;
        print!("{} ", n);
        n = hexadecimal;
        print!("{} ", n);
        n = octal;
        print!("{} ", n);
        n = binary;
        print!("{} ", n);
    }
}