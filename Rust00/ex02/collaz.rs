fn collatz(start: u32)
{
    if start < 1
   {return ();}
    let mut    buff;
    buff = start;
    while buff != 1
    {
        println!("{}", buff);
        if start % 2 == 0
            {buff = buff / 2;}
        else
            {buff = buff * 3 + 1;}
    }
        println!("{}", buff);
}

fn main()
{
    collatz(90);
}