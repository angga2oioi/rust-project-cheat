#[derive(Debug)]
pub struct Person {
    pub age: u8,
}

pub struct Child {
    pub person: Person,
    pub has_toy: bool,
}

impl Person {
    pub fn new(age: u8) -> Self {
        Person { age: age }
    }

    pub fn age(&self) -> u8 {
        self.age
    }
}

impl Child {
    pub fn new(age: u8, has_toy: bool) -> Self {
        Child { person: Person::new(age), has_toy: has_toy }
    }

    pub fn age(&self) -> u8 {
        self.person.age()
    }

    pub fn has_toy(&self) -> bool {
        self.has_toy
    }
}