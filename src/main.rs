use guess::front_house::{self,hosting};
fn main(){
    let hello =  hosting::echo_hello();
    println!("{}",hello);

    let s= String::from("hello,world");

    let  hello_2 = &s[0..4];

    let meal = front_house::Appetizer::Salad;

    front_house::against(meal);

    println!("{},",&hello_2);


    println!("{}",&hello_2);
}


