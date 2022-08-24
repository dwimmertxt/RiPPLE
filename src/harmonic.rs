pub fn fundamental(frequency_domain: &Vec<i32>) -> i32 {
    let max_magnitude = frequency_domain.iter().max();
    match max_magnitude {
        
        Some(max) => {
            let idx = frequency_domain.iter().position(|&r| r == *max).unwrap();
            idx as i32
        },
        None => 0
    }
}