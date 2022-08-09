pub mod bar;
pub mod bazz;


pub fn test_func(f: &dyn Fn()){
    f();
    println!("testfunc executed properly");
}