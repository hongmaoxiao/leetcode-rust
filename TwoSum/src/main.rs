use std::collections::HashMap;
fn main() {
    let nums = vec![-3,4,3,90].to_vec();
    let data = two_sum(nums, 0);
    println!("{},{}", data[0], data[1]);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        let curr = target - nums[i];
        let v = map.get(&curr);
        if v != None {
            match v {
                Some(&a) => {
                    return vec![i as i32, a as i32];
                },
                None => println!("not match")
            }
        } else {
            map.insert(nums[i], i as i32);
        }
    }
    return vec![];
}