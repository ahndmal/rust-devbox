
fn coll_two() -> usize {
    let mut arr:[i32; 6] = [23,31,12,4234,312,31];
    let mut leng = arr.len();
    arr[0] = 1;
    // arr = [i32; 8];
    let arr2 = &arr;
    let mut arr3 = arr2.clone();
    arr3[0] = 0;
    println!("{:?}", arr);
    println!("{:?}", arr2);
    println!("{:?}", arr3);
    leng = arr.len();
    return leng;
}

fn main_misc() {
    // fn concatt(a: &str, b: &str) -> &str {
    //     let c = concat!("{}{}", a, b);
    //         // let c = [a, b].join("");
    //     c
    // }
    // concatt("Hello ", "cat");

    let mut name: &str = "Murz";
    name = "Cat";
    let mut name_st = str::to_string(name);
    let mut name_2 = name.to_string();
    name_st.push('A');
    print!("{} {}", name, name_st);

    // let n = 6;
    // (0..n).collect::<Vec<u32>>();
    // let nums_arr = (0..n).collect::<Vec<u32>>();
    // for temp in nums_arr {
    //     println!("{}", temp);
    // }

    // positive_sum([1,-4,7,12])

}