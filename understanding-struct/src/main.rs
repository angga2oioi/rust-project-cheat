mod person;

use crate::person::{
    init_person,
    print_person
};

fn main() {
    
    let p= init_person();
    print_person(p);
}
