fn fizzbuzz()
{
    let mut a;

    a = 1;
    while a < 101
    {
        if a % 3 == 0 && a % 5 == 0
            {println!("FizzBuzz");}
        else if a % 3 == 0
            {println!("Fizz");}
        else if a % 5 == 0
            {println!("Buzz");}
        else
            {println!("{}", a);}
        a += 1;
    }
}

fn    main()
{
    fizzbuzz();
}