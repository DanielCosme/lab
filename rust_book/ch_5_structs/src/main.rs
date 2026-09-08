fn main() {
  users();
  rect();
}

fn rect() {
  #[derive(Debug)]
  struct Rectangle {
    width: u32,
    height: u32,
  }

  impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
      self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
      Rectangle {
        width: self.width.max(other.width),
        height: self.height.max(other.height),
      }
    }
  }

  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!("rect1 is {rect1:?}");
  println!("rect1 is {rect1:#?}");

  let scale = 2;
  let rect1 = Rectangle {
    width: dbg!(30 * scale),
    height: 50,
  };

  dbg!(&rect1);
  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  );

  let mut r = Rectangle {
    width: 1,
    height: 2,
  };
  let area1 = r.area();
  let area2 = Rectangle::area(&r);
  assert_eq!(area1, area2);

  let r = &mut Box::new(Rectangle {
    width: 1,
    height: 2,
  });
  let area1 = r.area();
  let area2 = Rectangle::area(&**r);
  assert_eq!(area1, area2);
}

fn users() {
  struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
  }

  let user_1 = User {
    active: false,
    username: "Daniel".to_string(),
    email: "undefined".to_string(),
    sign_in_count: 0,
  };

  let user_2 = User {
    email: "another@example.com".to_string(),
    username: "Anastasia".to_string(),
    ..user_1
  };

  println!(
    "name: {} - active: {} - email: {} {}",
    user_1.username, user_1.active, user_1.email, user_1.sign_in_count
  );
  println!(
    "name: {} - active: {} - email: {} {}",
    user_2.username, user_2.active, user_2.email, user_2.sign_in_count
  );
}
