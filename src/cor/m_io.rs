mod my_io {
    fn io_intro() {
        //intro
        let command_line: std::env::Args = std::env::args();
        for argument in command_line {
            println!("[{}]", argument);
        }
    }

    fn env_one() {
        print!("[{:?}]", std::env::var("abcd"));
        std::env::set_var("abcd", "This is the value");
        print!(" [{:?}]", std::env::var("abcd"));

        //

        print!("{}",
               if std::env::var("abcd").is_ok() {
                   "Already defined"
               } else {
                   "Undefined"
               });
        std::env::set_var("abcd", "This is the value");
        print!(", {}.", match std::env::var("abcd") {
            Ok(value) => value,
            Err(err) => format!("Still undefined: {}", err),
        });
    }
    
    // Result
    fn f(x: i32) -> Result<i32, String> {
        f4(f3(f2(f1(x)?)?)?)
    }
}