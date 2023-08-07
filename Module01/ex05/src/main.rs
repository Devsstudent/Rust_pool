
fn sort_slice(slice: &mut [i32])
{
	slice.sort();
}

fn main() {
	let mut a: [i32; 15] = [9, 9, 19, 56, 100, 1, 2, 3, 6, 1023, 90, 7, 8, 100, 90000];

	sort_slice(&mut a);
	println!("{:?}", a);
}
