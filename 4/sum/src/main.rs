fn get_sum(nums:&[u32]) -> Option<u32> {
    let mut sum : u32 = 0;
    for i in nums {
        match sum.checked_add(*i) {
            Some(r) => sum = r,
            //terminate early once overflow happens
            None => return None
        }
    }
    Some(sum)
}

fn main() {
    let vec1 = vec![1, 2, 3];
    println!("sum of vector is {:?}", get_sum(vec1.as_slice())); // expected to be Some(6)

    let vec2 = vec![u32::MAX, 1];
    println!("sum of vector is {:?}", get_sum(vec2.as_slice())); // expected to be None
}