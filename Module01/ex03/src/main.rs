fn add_vectors(a: [i32; 3], b: [i32; 3]) -> [i32; 3]
{
	let mut c: [i32; 3] = [0, 0, 0];

	c[0]  = a[0] + b[0];
	c[1]  = a[1] + b[1];
	c[2]  = a[2] + b[2];
	return c;
}

fn main() {
	let a = [1, 2, 3];
	let b = [2, 3, 4];
	let d = add_vectors(a, b);
	println!("{}", d[0]);
	println!("{}", d[1]);
	println!("{}", d[2]);

}
