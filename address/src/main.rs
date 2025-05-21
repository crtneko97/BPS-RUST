

fn main() 
{

    let arr: [i32; 3] = [1, 2, 3];

    for i in arr
    {
        println!("i = {}", i);
    }

    println!("i32 arr's address = {:p}", &arr);
    
    for i in &arr
    {
        println!("i32 = {} address = {:p}", i, i);
    }

    /*
      this prints following:
      arr's address = 0x7ffda2537c84
      i = 1 address = 0x7ffda2537c84
      i = 2 address = 0x7ffda2537c88
      i = 3 address = 0x7ffda2537c8c
     */

    let arr_two: [i16; 3] = [1, 2, 3];

    for i in &arr_two
    {
        println!("i16 i = {} adress = {:p}", i, i);
    }

    let arr_three: [i8; 3] = [1, 2, 3];

    for i in &arr_three
    {
        println!("i8 i = {} adress = {:p}", i, i);
    }


    let x: i64 =  60123012301230123;
    println!("{}", x);
    println!("{:p}", &x);
    
    let x : i16 = 5;
    println!("x (shadowed) = {}", x);
    println!("&x (shadowed) = {:p}", &x);

    let mut x: i16 = x + 1;
    println!("x (mut shadowed) = {}", x);
    println!("{:p}", &x);

    x += 2;
    println!("x (after +=) = {}", x);
    println!("{:p}", &x);
}
