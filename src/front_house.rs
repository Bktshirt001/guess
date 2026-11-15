pub mod hosting;

pub fn against(meal :Appetizer){
    match meal{
        Appetizer::Salad => println!("Salad"),
        Appetizer::Soup => println!("Soup"),    
        Appetizer::Sandwich => println!("Sandwich"),
    }
}

pub enum Appetizer{
    Soup,
    Salad,
    Sandwich,
}