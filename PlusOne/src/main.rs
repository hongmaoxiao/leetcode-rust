fn main() {
    let data = vec![9];
    let val = plus_one(data);
    println!("{:?}", val);
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut di_copy = digits.clone();
    let mut i = (di_copy.len() - 1) as isize;

    while i >= 0 && di_copy[i as usize] == 9 {
        di_copy[i as usize] = 0;
        i -= 1;
    }

    if i < 0 {
        di_copy.insert(0, 1);
    } else {
        di_copy[i as usize] += 1;
    }

    di_copy
}