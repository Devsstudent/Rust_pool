fn print_bytes(s: &str)
{
    for char in s.chars()
    {
        println!("{}", char);
    }
}

fn main()
{
        print_bytes("Yayayay");
}