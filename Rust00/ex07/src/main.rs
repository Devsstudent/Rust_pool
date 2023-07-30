fn main() {
    let mut price = -1;
    let real_price = ftkit::random_number(0..500);

    println!("Here is a skateboard.");
    while price != real_price
    {
        price = ftkit::read_number();
        if price > real_price
        {
            println!("A skateboard coasts less than that !");
        }
        else if price < real_price
        {
            println!("A skateboard coasts more than that !");
        }
    }
    println!("Congrates! That Skateboard coasts {}", price);
}
