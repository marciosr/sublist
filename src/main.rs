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
	} else if {
	
	}


	Comparison::Equal
}
