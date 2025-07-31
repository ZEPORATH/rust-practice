fn main() {
    println!("Hello, world!");
    let s1 = take_ownership();
    println!("{s1}");

    let s2 = String::from("World");
    give_ownership(s2);

    let (s3, size) = give_take_ownership(String::from("zeporath"));
    println!("{s3}, {size}");

    let s3 = String::from("github");
    let (s3, size) = take_ref(&s3);
    println!("{s3}, {size}");

    let mut s3 = String::from("github");
    let (s3, size) = take_mut_ref(&mut s3);
    println!("{s3}, {size}");

    let mut s3 = String::from("www");
    let s3 = multiple_mut_ref_errs(&mut s3);
    println!("{s3}");

    let _reference_to_nothing = dangle(); // this will give error
}

fn dangle() -> String {
    let s = String::from("hello");

    // &s // not allowed to return a dangling ref
    s
}

fn take_ownership() -> String {
    let s = String::from("Hello");
    s
}

fn give_ownership(s: String) {
    println!("{s}");
}

fn give_take_ownership(s: String) -> (String, usize) {
    println!("{s}");
    let len = s.len(); // We will need this, as tuple[0] will cause move of `s` ownership to result param making s.len() unavailable
    (s, len)
}

fn take_ref(s: &String) -> (&String, usize) {
    println!("{s}");
    (s, s.len())
}

fn take_mut_ref(s: &mut String) -> (&String, usize) {
    println!("{s}");
    s.push_str(".com");
    (s, s.len())
}

fn multiple_mut_ref_errs(s: &mut String) -> &String {
    let _s1 = &s; 
    let _s2 = &s;
    //s2 //returning s2 is not allowed as data is owned by s
}