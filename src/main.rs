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
    fn test(&self) {
        println!("TESTING");
    }

    fn panic_much(&self) -> ! {
        panic!("PAAAANIC");
    }
}

fn main() {
  let mut dog = Dog{name: "Doge".to_string()};
  let dog2 = Dog{name: "Doge 2".to_string()};

  println!("{}", dog.get_name());
  println!("{}", dog2.get_name());

  dog.test();
  dog2.test();

  dog.name = ":D".to_string();

  dog.set_name(":(".to_string());

  println!("{}", dog.name);

  dog.panic_much();
}
