#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>( first_list: &[T], second_list: &[T] ) -> Comparison {
	if first_list == &[] && second_list == &[] {
		Comparison::Equal
	} else if first_list == &[] && second_list != &[] {
		Comparison::Sublist
	} else if first_list != &[] && second_list == &[] {
		Comparison::Superlist
	} else if first_list == second_list {
		Comparison::Equal
	} else if first_list.len() == second_list.len() {
		Comparison::Unequal
	} else if first_list.len() < second_list.len() {
		if compare(first_list, second_list) == true {
			return Comparison::Sublist;
		};
		Comparison::Unequal
 	} else {
		if compare(second_list, first_list) == true {
			return Comparison::Superlist;
		};
		Comparison::Unequal
	}
}

fn compare<T: PartialEq>( small_list: &[T], bigger_list: &[T] ) -> bool {
	if bigger_list.windows(small_list.len()).any(|x| x == small_list) { true } else { false }
}
