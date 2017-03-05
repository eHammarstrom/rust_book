fn main() {
    let s = "hello";
    {
        let x = "hello";

        println!("{}", s);
        println!("{}", x);
    }
    // println!("{}", x); // err
    

    let mut s = String::from("hello");

    finish_sentence(&mut s);
    finish_sentence(&mut s);

    println!("{}", s);

    let hello_world = String::from("Hello cruel world!");

    let word = find_nth_word(&hello_world, 0);
    let word1 = find_nth_word(&hello_world, 1);
    let word2 = find_nth_word(&hello_world, 2);

    println!("{} : {} : {}", word, word1, word2);
}

fn finish_sentence(s: &mut String) {
    s.push_str(", yeah no.");
}

fn find_nth_word(s: &str, n: u32) -> &str {
    let bytes = s.as_bytes();
    let mut start_byte = 0;
    let mut end_byte = 0;
    let mut spc_count = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            spc_count += 1;

            if spc_count == n {
                start_byte = i+1;
            } else if spc_count == n+1 {
                end_byte = i;
            }
        }
    }

    if end_byte == 0 {
        end_byte = s.len()-1;
    }

    return &s[start_byte..end_byte];
}
