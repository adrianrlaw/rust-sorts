use std::cmp;

pub fn _quick_sort<T>(mut data: &Vec<T>, lo: usize, hi: usize)
where T: cmp::PartialOrd + Clone
{
	if lo >= hi {
		return;
	}

	let pivot = (lo + hi) / 2;
	let mut i = lo;
	let mut j = hi;

	loop {
		while data[i] < data[pivot] {
			i += 1;
		}
		while data[j] > data[pivot] {
			j -= 1;
		}
		if i >= j {
			break;
		}
		data.swap(i, j);
	}
}
