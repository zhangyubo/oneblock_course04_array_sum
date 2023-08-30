fn array_sum(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32;

    for &num in numbers {
        if let Some(result) = sum.checked_add(num) {
            sum = result;
        } else {
            return None; 
        }
    }

    Some(sum)
}

fn main() {
    let numbers = vec![2, 3, 4, 5, 6, 6];
    match array_sum(&numbers) {
        Some(result) => println!("求和结果：{}", result),
        None => println!("总和溢出，结果为：None."),
    }
}
