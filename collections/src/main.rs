use std::vec;
use std::collections::HashMap;
// enum Addict {
//     Int(i32),
//     Float(f64),
//     Text(String)
// }
fn countword(str: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for w in str.split_whitespace() {
        let count = map.entry(w.to_string()).or_insert(0);
        *count += 1;
    }
    map
}
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v = vec![1,2,3];
    // v.push(4);

    // let third = &v[2];
    // let fourh = v.get(3).expect("no such element");
    println!("{:?}", v);
    // println!("third elm is: {}", third);
    // println!("fourth is {:?}", fourh);

    // for i in &v {
    //     print!("{i}");
    // }

    // to change
    // for i in &mut v {
    //     *i *= 2;
    // }

    // let row = vec![
    //     Addict::Int(2),
    //     Addict::Float(78.00),
    //     Addict::Text(String::from("In the downfall"))
    // ];

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; 

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{s1}-{s2}-{s3}");

    let hello = "Здравствуйте";
    // for i in hello.chars() {
    //     println!("{i}");
    // }
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name: String = String::from("Blue");
    let score: i32 = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    scores.entry(String::from("Red")).or_insert(50);
    let word_count = countword(&hello);
    println!("{word_count:?}");

}
