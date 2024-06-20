use std::vec;

// enum Addict {
//     Int(i32),
//     Float(f64),
//     Text(String)
// }

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v = vec![1,2,3];
    // v.push(4);

    let third = &v[2];
    let fourh = v.get(3).expect("no such element");
    println!("{:?}", v);
    println!("third elm is: {}", third);
    println!("fourth is {:?}", fourh);

    for i in &v {
        print!("{i}");
    }

    // to change
    // for i in &mut v {
    //     *i *= 2;
    // }

    // let row = vec![
    //     Addict::Int(2),
    //     Addict::Float(78.00),
    //     Addict::Text(String::from("In the downfall"))
    // ];

}
