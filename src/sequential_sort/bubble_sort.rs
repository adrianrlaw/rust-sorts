use std::cmp;

pub fn _bubble_sort<T>(data: Vec<T>) -> Vec<T>
where T: cmp::PartialOrd + Clone
{
	let mut result = data;

	let mut swapped: bool = false;
	let len = result.len();
	let mut sublen = result.len();

	for _ in 0..len {
		for j in 1..sublen {
			if result[j - 1] > result[j] {
				result.swap(j - 1, j);
				swapped = true;
			}
		}
		if !swapped {
			break;
		}
		sublen -= 1;
	}
	result
}