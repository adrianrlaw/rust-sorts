use std::cmp;

use self::bubble_sort::_bubble_sort;

mod bubble_sort;
pub fn bubble_sort<T>(data: &[T]) -> Vec<T>
where T: cmp::PartialOrd + Clone
{
	_bubble_sort(data.into())
}