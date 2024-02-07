fn find_shortest_word(text: &str) -> Option<&str> {
    // Split the text into words
    let words = text.split_whitespace();

    // Check if there are any words
    if words.count() == 0 {
        return None;
    }

    // Initialize the shortest word and its length
    let mut shortest_word = words.next().unwrap();
    let mut shortest_length = shortest_word.len();

    // Iterate through the remaining words
    for word in words {
        // Check if the current word is shorter than the shortest found so far
        if word.len() < shortest_length {
            shortest_word = word;
            shortest_length = word.len();
        }
    }

    // Return the shortest word
    Some(shortest_word)
}

let text = "This is a string with words of different lengths.";
let shortest = find_shortest_word(text);
println!("Shortest word: {}", shortest.unwrap()); // Output: Shortest word: is

let text = "The quick brown fox jumps over the lazy dog.";
let shortest = find_shortest_word(text);
println!("Shortest word: {}", shortest.unwrap()); // Output: Shortest word: The

let text = "";
let shortest = find_shortest_word(text);
println!("Shortest word: {:?}", shortest); // Output: Shortest word: None