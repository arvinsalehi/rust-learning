use std::ops::{Add, Div};

fn max<T: PartialOrd + Copy>(vec: Vec<T>) -> (T, i32) {
    let mut max: T = vec[0];
    let mut max_index = 0;
    for (index, number) in vec.iter().enumerate() {
        if *number > max {
            max = *number;
            max_index = index;
        }
    }
    (max, max_index as i32)
}

#[derive(Debug)]
struct Point<T, U, Y> {
    x: T,
    y: U,
    z: Y,
}

impl<T, U, Y> Point<T, U, Y> {
    fn x(&self) -> &T {
        &self.x
    }
    fn mixup<T1, U1, Y1>(
        self,
        other: Point<T1, U1, Y1>,
        indx: i32,
    ) -> Result<Point<T1, U, Y>, &'static str>
    where
        T: Clone,
        U: Clone,
        Y: Clone,
        T1: Clone,
        U1: Clone,
        Y1: Clone,
    {

        // QUESTION |
        //          
        // match indx {
        //     0 => Ok(Point { x: other.x.clone(), y: self.y , z: self.z}),
        //     1 => Ok(Point { x: self.x, y: self.y, z: self.z}),
        //     2 => Ok(Point { x: self.x, y: self.y, z: self.z}),
        //     _ => Err("invalid index"),
        // }

        Ok(Point {
            x: other.x,
            y: self.y,
            z: self.z,
        })
    }
}

fn main() {
    let vec = vec![10, 20, 0, 0, 1, 2, 22];

    let (max, index) = max(vec);
    println!("max is {} and its index is {}", max, index);

    let point = Point {
        x: 10.2,
        y: 1.0,
        z: -10.2,
    };

    println!("{:?}", point);
}
