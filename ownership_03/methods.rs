struct Rectangle {
  width: u32,
  height: u32
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50
  };

  println!("The area of the rectangle is {} square pixels",
    rect1.area()
  );

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
}

impl Rectangle {    
  fn area(self: &Self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other_rect: &Rectangle) -> bool {
    let me: u32 = self.area();
    let other = other_rect.area();

    me > other
  }

  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size
    }
  }
}