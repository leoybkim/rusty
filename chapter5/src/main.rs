struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("-------------- 5.1 Defining and Instantiating Structs --------------");
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user1.email: {}", user1.email);
    println!("user2.username: {}", user2.username);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;

    println!("-------------- 5.2 An Example Program Using Structs --------------");
    // let width1 = 30;
    // let height1 = 50;

    // let rect1 = (30, 50);

    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        // area(rect1)
        area(&rect1)
    );

    println!("-------------- 5.3 Methods --------------");
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!(
        "The square has width: {} and height: {}",
        sq.width, sq.height
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}
//
//fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//}
//
fn area(rectangle: &Rectangle) -> u32 {
    // We want to borrow the struct rather than take ownership of it.
    // This way, main retains its ownership an can continue using rect1.
    rectangle.width * rectangle.height // accessing fields of a borrowed struct instance does not
    // move the field values
}
