

fn main() 
{
    for i in 1..10
    {
        println!("{} \tadd {:p}", i, &i);
    }

    for i in 1..=10
    {
        println!("{}", i);
    }

    let arr = [10, 20, 30];

    for value in arr
    {
        println!("v = {}", value);
    }

    for (idx, value) in arr.iter().enumerate()
    {
        println!("arr[{}] = {} \taddr = {:p}", idx, value, value);
    }

    let mut n = 3;
    while n > 0
    {
        println!("n = {}", n);
        n -= 1;
    }
    println!("loop done");



    let mut count = 0;

    loop
    {
        count += 1;

        if count == 3
        {
            println!("Reached 3, breaking..");
            break;
        }
    }


    let result = loop
    {
        if count >= 5
        {
            break count * 2;
        }
        count += 1;
    };

    println!("result = {}", result);
}
