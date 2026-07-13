use std::io;
use rand::Rng;
fn main() {
    let mut guess = String::new();

    guess = rand::thread_rng().gen_range(1..100).to_string();
    loop{
        let mut input = String::new();

        println!("请输入一个数！：{}",&mut input);

        io::stdin()
        .read_line(&mut input)
        .expect("读取失败！");

        if input.trim() == guess.trim(){
            println!("恭喜您，猜对了！");
        }else if input.trim() > guess.trim() {
            println!("您输入的数太大了！");
        }else {
            println!("您输入的数太小了！");
        }

        println!("您输入的数是：{}",&mut input);
    
    }
}
