fn split_once_at_null(s: &str) -> (&str, &str)
{
	let str_as_bytes = s.as_bytes();
	let mut i = 0;
	for chars in str_as_bytes
	{
		println!("{}", chars);

		if *chars == 0
		{
			return s.split_at(i);
		}
		i += 1;

	}
	panic!("No 0 bytes founded");

}

fn main() {
	//let a: &str = "";
	//let a: &str = "test\0test";
	//let a: &str = "test mdrr \0 nice \0\0";
	let a: &str = "bnlbllblbllbbl";
	//let a: &str = "\0";

	let (b, c) = split_once_at_null(a);
	println!("{:?}, {:?}",b, c);
}
