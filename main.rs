fn sum_even_numbers(numbers: &Vec<i32>) -> i32 {
    // Use the filter function to create a new iterator of even numbers
    let even_numbers = numbers.iter().filter(|&n| n % 2 == 0);

    // Use the fold function to sum the even numbers
    even_numbers.fold(0, |acc, n| acc + n)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum = sum_even_numbers(&numbers);

    println!("Sum of even numbers: {}", sum);
}
