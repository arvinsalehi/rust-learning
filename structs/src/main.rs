struct User {
    age: i32,
    name: String,
    smart: bool,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn build_user(name: String) -> User {
    User {
        age: 21,
        name,
        smart: false,
    }
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

    println!("userage: {} username: {} smart:{}", user3.age, user3.name,user3.smart)

}
