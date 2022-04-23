#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// pub fn sublist<T: PartialEq>(_first_list: &[T],
// 							 _second_list: &[T] 
// 							 ) -> Comparison {
//     unimplemented!(
//     	"Determine if the first list is equal to, sublist of,
//     	superlist of or unequal to the second list."
//     );
// }

pub fn sublist<T: PartialEq>(
	first_list: &[T],
	second_list: &[T] 
) -> Comparison {

	let res = first_list.eq(second_list);



}
