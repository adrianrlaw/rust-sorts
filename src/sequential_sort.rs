use std::cmp;

use self::{bubble_sort::_bubble_sort, quick_sort::_quick_sort};

mod bubble_sort;
pub fn bubble_sort<T>(data: &[T]) -> Vec<T>
where T: cmp::PartialOrd + Clone
{
	_bubble_sort(data.into())
}

mod quick_sort;
pub fn quick_sort<T>(data: &[T]) -> Vec<T>
where T: cmp::PartialOrd + Clone
{
	let mut result: Vec<T> = data.into();
	let end = result.len() - 1;
	_quick_sort(&mut result, 0, end);
	result
}