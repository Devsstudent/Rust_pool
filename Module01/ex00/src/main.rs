
fn add_assign(a: &mut i32, b: i32)
{
	*a = b;
}

fn main()
{
	let mut a:i32 = 3;
	let b: i32 = 4;

	println!("start value a: {}, b: {}", a, b);
	add_assign(&mut a, b);
	println!("new value a: {}, b: {}", a, b);
//	add_assign(&a, b);
}
