use std::cmp;

pub fn _quick_sort<T>(data: &mut Vec<T>, lo: usize, hi: usize)
where T: cmp::PartialOrd + Clone
{
	if lo >= hi {
		return;
	}

	let pivot: usize = (lo + hi) / 2;
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

	_quick_sort(data, lo, pivot);
	_quick_sort(data, pivot + 1, hi);
}
