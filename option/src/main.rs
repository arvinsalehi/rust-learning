use core::num;

fn inc(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn eval_num(num: Option<i32>) {
    match num {
        None => println!("Fool your self bitch"),
        Some(num) => match num {
            66 => println!("Boring"),
            67 => println!("Eeeh"),
            68 => println!("hmmmm"),
            69 => println!("Nice"),
            _ => println!("Don't give a fuck"),
        },
    }
}

fn main() {
    let number = Some(68);
    let number = inc(number);
    eval_num(number);
}
