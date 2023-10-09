use std::default::Default;

struct A {
    x: i32,
    y: i32,
}

// In python
// class D:
//     def __init__(self, x, y, z=None):
//          self.x = x
//          self.y = y
//          self.z = z if z is not None else 0
//         pass

// we can do the above directly, as rust doesn't support classes
// so we use a struct and implement Default for having a
// derive debug is to help in printing the values of D
#[derive(Debug)]
struct D {
    x: i32,
    y: i32,
    z: i32,
}

impl D {
    fn new(x: i32, y: i32, z: Option<i32>) -> Self {
        match z {
            Some(z) => Self { x, y, z },
            None => Self { x, y, z: 0 },
        }
    }
}

impl Default for D {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

fn main() {
    // let mut a = A { x: 1, y: 2 };

    // println!("{}", a.x);
    // a.x += 2;

    // println!("{}", a.x)

    // force the user to provide x,y input
    let d1 = D::new(1, 2, None);

    // handle the user's any x,y,z input
    let d = D {
        x: 1,
        y: 2,
        ..D::default()
    };

    println!("{:?}", d1);
    println!("{:?}", d)
}
