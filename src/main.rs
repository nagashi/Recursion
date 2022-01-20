fn count_down(nbr: i32) -> Vec<i32> {
    match nbr < 1 {
        true => Vec::new(),
        false => {
            let mut vec = count_down(nbr -1);
            vec.insert(0, nbr);
            vec
        }
    }
}

fn main() {
    println!("Countdown {:?}", count_down(10));
}