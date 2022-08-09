mod foo;
use crate::foo::*;
use crate::foo::bar::*;
use crate::foo::bazz::*;


fn two_func(){
    println!("two_Func executed properly")
}

fn main() {
    
    test_func(&two_func);

    bar();
    bazz();

    println!("Hello, world!");

}
