use std::io::{self,Write};
fn main() {
    loop{
    print!(">");
    io::stdout().flush();
    let mut input=String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let val:Vec<&str>=input.split_whitespace().collect();
    if val.len()>=3 {
    let value_one=val[0];
    let value_two=val[1];
    let value_three=val[2];
    
    let val_1=value_one.trim().parse::<f32>().unwrap();
    let operator=value_two.trim().parse::<char>().unwrap();
    let val_2=value_three.trim().parse::<f32>().unwrap();
    
    if val_1!=0.0{
    if operator=='+'{
        println!("{}",val_1+val_2);
    }
    else if operator=='-'{
        println!("{}",val_1-val_2);
    }
    else if operator=='*'{
        println!("{}",val_1*val_2);
    }
    else if operator=='/'{
        if val_1%val_2==0.0{
        println!("{}",val_1/val_2);
    }
else {
    println!("{}",val_1 as f32/val_2 as f32);
}}
    else if operator=='^'{
        let mut pow=1;
        for _i in 1..((val_2 as i8)+1){
            pow=pow*val_1 as i32;
        }
        println!("{}",pow);
    }
    else{
        println!("Enter valid operator and values.");
    }
    }
    else{
        println!("Bye");
        break;
    }
}
else{
    println!("Enter values with space between them.");
}
    
    }

}
/*******************************************************Question 2 */

fn main() {
    let x = || println!("Hello world");//make a closure which takes no argument and prints hello world
    x();
}
fn main() {
    let x = |z:u32| z+1;//Make a closure which takes one u32 data type as argument and returns with adding 1 to it.
    y = 4;
    println!("The function returns: {}",x(y)); 
}
fn main() {
    let c = 1;
    let x = || c+1;//Make a closure which captures value of variable "c" from environment and change the value of c with adding 1.
    x();
    println!("The new value of c is: {}",c); // should print 2
}
// Write a function which accepts a closure, and in the funciton body, it calls the closure. The closure accepts no argument and returns nothing. What should the closure be about? Be creative!
fn main() {
    let func=|| {println!("hello");};
    closure(func);
}
fn closure<T:Fn()>(func:T){
        func();
}
// Write a function which expects a closure as an argument and in the funciton body, it calls the closure. The closure expects u32 argument and returns the u32 value. The closure adds 1 to the argument and returns it.
fn main() {
    let func=|z| {z+1};
    closure(func);
}
fn closure<T:Fn(u32)->u32>(func:T){
        let y=func(87);
        println!("{}",y);
}
 
/*****************************Question 3  */
#[derive(Debug)]
struct Price<T> 
    where T:Fn(i32)->i32{
      apple_price:T,
      mango_price:T,
}

impl<T> Price<T>
where T: Fn(i32)->i32{
    fn add(apple_price:T,mango_price:T)->Price<T>{
        Price {
            apple_price,
            mango_price,
        }
    }
}
fn main(){
    let y=104;
    let z=450;
    let p=|x|{x+1};
    let price_all=Price{apple_price:p, mango_price:p};
    print!("{} {}",Price::add((price_all.apple_price)(y),(price_all.mango_price)(z)));
    print!("{}",Price::add(p(y),p(z)));
}


/*************************************Question 4 */
struct Child{
    name: String,
}

pub trait Quality{
    fn primary_education(&self)->String;
    fn bilingual(&self)->String;
}
impl Quality for Child{
    fn primary_education(&self)->String{
        format!("{} has completed Primary Education.",self.name)
    }
    fn bilingual(&self)->String{
        format!("{} is bilingual.",self.name)
    }
}

fn adopt<T:Quality>(child_1:T,child_2:T){
        println!("You can adopt both these children.");
        println!("{} ,{}\n{} ,{}",child_1.primary_education(),child_1.bilingual(),child_2.primary_education(),child_2.bilingual());
}

fn main(){
    let child_one=Child{name: String::from("child1")};
    let child_two=Child{name: String::from("child2")};
    adopt(child_one,child_two);
}

/*********************************************Question 5 */
1. Rust’s closures are anonymous functions
2. you can save in a variable or pass as arguments to other functions.
3. You can create the closure in one place and then call the closure to evaluate it in a different context. 
4. Closures can capture values from the scope in which they’re defined.
