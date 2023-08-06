fn smallest_subslice<'a>(slice: &'a [u32], treshold: &u32) -> &'a [u32]
{
	let mut i: usize = 0;
	let mut j: usize = 0;
	let mut sum = 0;
	let mut buff_round = -1;
	let mut rounds = 0;
	let mut indexes: [usize;2] = [0, 0];
	while i < slice.len()
	{
		sum = sum + slice[i];
		if sum > *treshold
		{
			if buff_round == -1 || rounds < buff_round
			{
				println!("{}", buff_round);
				buff_round = rounds;
				indexes[0] = j;
				indexes[1] = i + 1;
				println!("{:?}", indexes);
			}
			rounds = 0;
			i = j + 1;
			j = i;
			sum = 0;
			continue ;
		}
		rounds = rounds + 1;
		i = i + 1;
	}
	if indexes[0] == 0 && indexes[1] == 0
	{
		return &[];
	}
	else
	{
		return &slice[indexes[0]..indexes[1]];
	}
	//Compare all value to find to 
}

fn main() {
 let array = [1, 2, 3, 4, 1, 2, 12];
 let result;
	{
		let treshold = 12;
		result = smallest_subslice(&array, &treshold);
	}
assert_eq!(result, &[]);
}
