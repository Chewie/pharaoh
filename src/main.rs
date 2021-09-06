//use std::panic;
use std::error::Error;

//fn run_test() {
    //let prev_hook = panic::take_hook();
    //panic::set_hook(Box::new(|_| {}));
    //let result = panic::catch_unwind(|| {
        //assert_eq!(4, 2);
    //});
    //panic::set_hook(prev_hook);
    //match result {
        //Ok(_) => (),
        //Err(e) => println!("{}", e.downcast::<String>().unwrap())
    //}
//}


fn main() -> Result<(), Box<dyn Error>>{
    pharaoh::run()
}
