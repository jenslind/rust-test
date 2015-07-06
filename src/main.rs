struct Dog {
  name: String
}

trait HasName {
    fn get_name(&self) -> &String;
    fn set_name(&mut self, name: String);
}

impl HasName for Dog {
  fn get_name(&self) -> &String {
	&self.name
  }

  fn set_name(&mut self, name: String) {
      self.name = name;
  }
}

impl Dog {
    fn new(dog_name: String) -> Dog {
        Dog {name: dog_name}
    }

    fn test(&self) {
        println!("TESTING");
    }

    // Paaaaanic returns nothing
    fn panic_much(&self) -> ! {
        panic!("PAAAANIC");
    }

    // Static method
    fn test2() -> i32 {
        28
    }
}

fn main() {
  let mut dog = Dog{name: "Doge".to_string()};
  let dog2 = Dog{name: "Doge 2".to_string()};
  let dog3 = Dog::new("Doge 3".to_string());

  println!("{}", dog.get_name());
  println!("{}", dog2.get_name());
  println!("{}", dog3.get_name());

  dog.test();
  dog2.test();

  dog.name = ":D".to_string();

  dog.set_name(":(".to_string());

  println!("{}", dog.name);

  let magic_number = Dog::test2();
  println!("Magic number: {}", magic_number);

  dog.panic_much();
}
