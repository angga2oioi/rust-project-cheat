pub mod person;
use person::{
    Person,
    Child,
};

pub fn init_person() -> Person{
    return Person::new(42);
}

pub fn init_child() -> Child{
    return Child::new(12,false);

}

pub fn print_person(p : Person){
    println!("{:?}",p);
}