fn count_down(nbr: i32) -> Vec<i32> {
    match nbr < 1 {
        true => Vec::new(),
        false => {
            let mut vec = count_down(nbr - 1);
            vec.insert(0, nbr);
            vec
        }
    }
}

fn num_range(begin_num: i32, end_num: i32) -> Vec<i32> {
    match begin_num > end_num {
        true => Vec::new(),
        false => {
            let mut vec = num_range(begin_num + 1, end_num);
            vec.insert(0, begin_num);
            vec
        }
    }
}

fn main() {
    println!("\nCountdown {:?}", count_down(10));
    println!("\nNumber Range {:?}\n", num_range(-4, 10));
}
