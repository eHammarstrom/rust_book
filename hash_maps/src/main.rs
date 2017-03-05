use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 1);
    scores.insert(String::from("Red"), 3);
    scores.insert(String::from("Blue"), 2);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![2, 3];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("Team Red: {}", scores.get(&String::from("Red")).unwrap());
}
