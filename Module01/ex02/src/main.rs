fn min<'a>(a: &'a i32, b: &'a i32) -> &'a i32
{
	if a < b
	{
		return a;
	}
	return b;
}
/*
fn min(a: &i32, b: &i32) -> &i32
{
	if a < b
	{
		return a;
	}
	return b;
}*/

fn main()
{
	let a: i32 = 2;
	let b: i32 = 4;

	println!("{}", *min(&a, &b));
}
