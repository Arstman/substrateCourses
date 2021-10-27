fn main() {
    // 3 slice of u32, the sum of first 2 slice should be Some(u32) and the sum of sum_none will be
    //   overflow and return None.
    let sum_a = [1u32, 2, 4, 5];
    let sum_b = [8u32; 20];
    let sum_none = [1, 32, 64, 128, 4294967295];

    println!("The Sum of slice sum_a is {:?}", checked_sum(&sum_a));
    println!("The Sum of slice sum_b is {:?}", checked_sum(&sum_b));
    println!("The Sum of slice sum_none is {:?}", checked_sum(&sum_none));
}

fn checked_sum(c: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for num in c.iter() {
        //checked_add is method of u32 from std
        match sum.checked_add(*num) {
            Some(r) => sum = r,
            None => return None,
        }
    }
    return Some(sum);
}
