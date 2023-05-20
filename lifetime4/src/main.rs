fn main() {
    let sentence = String::from("ulimate chance to win");
    let fword = first_word(&sentence); 
    println!("First Word: {}", fword);
}




fn first_word(s: &str) -> &str {

    let bytest = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item = b' ' {
            return &s[0..i];
        }

    }

    &s[..]    
} 







