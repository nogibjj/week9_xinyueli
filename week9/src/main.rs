struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // This is a method that calculates the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This is a method that checks if the rectangle can fit inside another rectangle
    fn fits_inside(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height < other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 60 };

    println!("The area of rect1 is {} square pixels.", rect1.area());

    if rect1.fits_inside(&rect2) {
        println!("rect1 fits inside rect2!");
    } else {
        println!("rect1 does not fit inside rect2.");
    }

    let mut rect3 = Rectangle { width: 10, height: 10 };

    // This loop will increase the size of rect3 until it can fit inside rect2
    loop {
        if rect3.fits_inside(&rect2) {
            println!("The final size of rect3 is {}x{}.", rect3.width, rect3.height);
            break;
        }

        rect3.width += 5;
        rect3.height += 5;
    }
}
