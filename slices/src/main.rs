fn first_word_size(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (_i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..1]
        }
    }
    &s
}

fn main() {
    let mut s = String::from("Hello World!");
    let word = first_word_size(&s);
    println!("size: {word}");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}");
    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole.
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    println!("{word}");
    let word = first_word(&my_string_literal[..]);
    println!("{word}");
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{word}");
    s.clear();
}
