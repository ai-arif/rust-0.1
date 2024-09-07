fn main() {
    //data types
    // let mut my_num:u16 = 10; //mutable variable
    // my_num = 20;
    // println!("My number is: {}", my_num);

    //&str -> Fixed sized string, stored in special memory (stack)

    // let my_name:&str = "AI Arif";
    // println!("{}", my_name);

    //String -> Dynamic sized string, stored in heap

    let mut my_name:String = String::from("AI Arif");
    my_name.push_str(" Islam");
    println!("{}", my_name);
}
 