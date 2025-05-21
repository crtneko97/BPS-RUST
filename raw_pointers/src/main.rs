
fn main() 
{
    let mut x = 1;

    {
    // start immuteable borroww
    let r_p_1: &i8 = &x;
    println!("raw pointer 1 -> {}", *r_p_1);
    println!("{:p}", r_p_1);
    }// dropp r_p_1, immutable borrows ends

    {
    // now we can take a mutable borrow
    let r_p_2: &mut i8 = &mut x;
    *r_p_2 += 10;
    println!("{}", x);
    }


    let mut y = 11;

    let r_p_y1: *const i32 = &y as *const i32;
    let r_p_y2: *mut i32 = &mut y as *mut i32;

    // we can always print their addresses safely
    println!("raw pointer y 1 = {:p}", r_p_y1);
    println!("raw pointer y 2 = {:p}", r_p_y2);


    // but to actually read or writewe need to unsafe
    unsafe
    {
    // Dereference the raw pointer to *read*
    println!("*r_p_y1 (before) = {}", *r_p_y1);


    // Dereference r_p_y2
    *r_p_y2 += 10;
    println!("y (after *r_p_y2 += 10) = {}", *r_p_y2);
    }



    let mut b: i32 = 20;

    // first to *const i32, then to *const i8
    let r1_bytes: *const i8 = (&b as *const i32) as *const i8;
    let r2_bytes: *mut i8   = (&mut b as *mut i32)   as *mut i8;

    println!("r1_bytes addr = {:p}", r1_bytes);
    println!("r2_bytes addr = {:p}", r2_bytes);

    unsafe 
    {
        // reading a single byte off the front of the i32
        println!("first byte of b = {}", *r1_bytes);
        // writing a byte (you’re only changing the lowest‐order byte here)
        *r2_bytes = *r2_bytes.wrapping_add(1);
        println!("b now = {}", b);
    }
     
}

