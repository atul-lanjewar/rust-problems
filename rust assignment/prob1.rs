fn is_palindrome(text: &str) -> bool {
    let mut chars = text.chars().filter(|c| c.is_alphanumeric()).map(|c| c.to_ascii_lowercase());
    while let Some(first) = chars.next() {
        if chars.next().back() != Some(first) {
            return false;
        }
    }
    true
}

println!("racecar is a palindrome: {}", is_palindrome("racecar"));  // Output: true
println!("hello is not a palindrome: {}", is_palindrome("hello"));  // Output: false
println!("A man, a plan, a canal: Panama is a palindrome: {}", is_palindrome("A man, a plan, a canal: Panama"));  // Output: true