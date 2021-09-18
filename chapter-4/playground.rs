fn main() {
    let s1 = String::from("hello world from here");
    // let s2 = takes_ownership(&mut s1);

    println!("{}", first_word(&s1));
   
}

// fn takes_ownership(s: &mut String) {
//     println!("I've taken ownership");
//     s.push_str(" from the other side");
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}