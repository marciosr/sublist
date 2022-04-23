use sublist::{sublist, Comparison};

fn main () {
		
}

// #[derive(Debug, PartialEq)]
// pub enum Comparison {
//     Equal,
//     Sublist,
//     Superlist,
//     Unequal,
// }

pub fn sublist<T: PartialEq>(
	first_list: &[T],
	second_list: &[T] 
) -> Comparison {

	//let res = first_list.eq(second_list);
		
	if first_list == &[] && second_list == &[] {
		Comparison::Equal
	} else if first_list == &[] && second_list != &[] {
		Comparison::Sublist
	} else if first_list != &[] && second_list == &[] {
		Comparison::Superlist
	} else if first_list == second_list {
		Comparison::Equal
	} else if first_list.len() < second_list.len() {
		Comparison::Unequal ///TODO: to include the comparisson function here!
 	} else {
		Comparison::Unequal ///TODO: to include the comparisson function here!
	}
}

fn compare<T: PartialEq>( small_list: &[T], bigger_list: &[T] ) -> bool {
	let small_size: usize = small_list.len();
	let mut slice = small_list.chunks_exact(small_size);

}
