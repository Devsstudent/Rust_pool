fn get_string(key: &i32) -> &str
{
	if *key < 0 || *key > 9
	{
		panic!("Key is not between 0 and 9");
	}
	static res: &str = "ice";
	return res;
}

fn main() {
	let result;

		let key = ftkit::random_number(0..10);
		result = get_string(&key);
	println!("{result}");
}
