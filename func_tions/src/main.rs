
fn greet(name: &str)
{
    println!("Hello, {}!})", name, &name);
}

fn take_ownership(s: String)
{
    println!("Got ownership of: {}", s);
}

fn borrow_but_not_take_ownership(s: &String)
{
    println!("Just borrowing: {}", s);
}

fn square(x: i32) -> i32
{
    x * x
}

fn swap(a: i32, b: i32) -> (i32, i32)
{
    (b, a)
}


fn main() 
{
    greet("Simon");
    let s = String::from("Hello");
    borrow_but_not_take_ownership(&s);
    println!("Still owns: {}", s);
    take_ownership(s);
    let nine = square(3);
    println!("{}", nine);
    let (x, y) = swap(1, 2);
    println!("x = {} y = {}", x, y);
}
