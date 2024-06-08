struct User {
    age: i32,
    name: String,
    smart: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rect(u32, u32);

fn build_user(name: String) -> User {
    User {
        age: 21,
        name,
        smart: false,
    }
}

fn rect_area(rect: &Rect) -> u32 {
    rect.0 * rect.1
}
fn main() {
    let mut user1 = User {
        age: 23,
        name: String::from("User 1"),
        smart: false,
    };

    user1.name = String::from("New User 1");

    let user2 = build_user(String::from("Init"));

    let user3 = User {
        smart: true,
        ..user2
    };
    println!(
        "user age: {} username: {} smart:{}",
        user3.age, user3.name, user3.smart
    );

    let color = Color(128, 128, 128);
    let point = Point(128, 128, 128);

    let rect: Rect = Rect(12, 13);
    println!(
        "Color is: ({}, {}, {}) and point is : ({},{},{})",
        color.0, color.1, color.2, point.0, point.1, point.2
    );

    println!("Area of rect is: {}", rect_area(&rect));

    dbg!(color);
    dbg!(rect);
}
