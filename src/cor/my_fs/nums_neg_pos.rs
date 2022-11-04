
fn closure_refs() {
    // print!("{} ", std::mem::size_of::<i32>());
    // print!("{} ", std::mem::size_of_val(&12));
    //
    // let fl =  File::open("lorem.txt");
    // let mut buf = "";
    // match fl {
    //     Ok(data) => {
    //         data.read_to_string(&mut buf);
    //         println!("{:?}", data)},
    //     Err(e) => {println!("ERROR")}
    // }
}

fn positive_sum(slice: &[i32]) -> i32 {
    // let mut nums: Vec<i32> = vec![];
    let mut summ = 0;
    for a in slice {
        if a > &0 {
            summ += a;
        }
    }
    return summ;
}