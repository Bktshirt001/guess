// use crate::front_of_house::Breakfast;

// mod  back_house{
//     pub enum Appetizer{
//         Soup,
//         Salad,
//     }
// }


// mod front_of_house{
//     pub struct Breakfast{
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast{
//         pub fn summer(toast: &str) -> Breakfast{
//             Breakfast{
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }

//     pub mod hosting{
//         pub fn hello_world(){
//             println!("hello");
//         }
//     }
// }

// pub fn eat_at_re(){
//     crate::front_of_house::hosting::hello_world();

//     let anna :Breakfast =  front_of_house::Breakfast::summer("Rye");
//     println!("Anna's breakfast: {}", anna.toast);
// }

pub mod front_house;
pub mod fron;


