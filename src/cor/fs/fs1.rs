mod fs {

    use std::io::Read;

    fn reading() {
        let mut file = std::fs::File::open("lorem.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        print!("{}", contents);
    }

    fn writing() {
        use std::io::Write;
        let mut file = std::fs::File::create("data.txt").unwrap();
        file.write_all("eè€".as_bytes()).unwrap();
    }
    
    fn copy_file() {
        use std::io::Read;
        use std::io::Write;
        
        let mut command_line: std::env::Args = std::env::args();
        command_line.next().unwrap();
        let source = command_line.next().unwrap();
        let destination = command_line.next().unwrap();
        let mut file_in = std::fs::File::open(source).unwrap();
        let mut file_out = std::fs::File::create(destination).unwrap();
        let mut buffer = [0u8; 4096];
        loop {
            let nbytes = file_in.read(&mut buffer).unwrap();
            file_out.write(&buffer[..nbytes]).unwrap();
        }
    }

    fn count_lines(pathname: &str)
                   -> Result<(u32, u32), std::io::Error> {
        use std::io::BufRead;
        let f = std::fs::File::open(pathname)?;
        let f = std::io::BufReader::new(f);
        let mut n_lines = 0;
        let mut n_empty_lines = 0;
        for line in f.lines() {
            n_lines += 1;
            if line?.trim().len() == 0 {
                n_empty_lines += 1;
                }
            }
        Ok((n_lines, n_empty_lines))
    }
}

