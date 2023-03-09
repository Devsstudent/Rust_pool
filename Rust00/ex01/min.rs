fn min(a: i32, b: i32) -> i32
{
    if a < b {
        return a;}
    else{
        return b;}
}

fn main()
{
    println!("{}", min(1, 2));
}