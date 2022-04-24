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
		    println!("{:?}", compare2(first_list, second_list));
			return Comparison::Sublist;
		};
		Comparison::Unequal
 	} else {
		if compare(second_list, first_list) == true {
		    println!("{:?}", compare2(second_list, first_list));
			return Comparison::Superlist;
		};
		Comparison::Unequal
	}
}

#[allow(unused)]
fn compare2<T: PartialEq>( small_list: &[T], bigger_list: &[T] ) -> bool {
    let vec_small = vec!(small_list);
    let mut vec_bigger = vec!(bigger_list);
    // let mut vec_result = Vec::new();
    // let i: usize = 0;

    vec_bigger.retain(|&x| {
        let mut a = false;
        for i in &vec_small {
            if x == *i{
                a = true;
            }
        }
        a
    });

    if vec_bigger.len() > 1 {
        true
    } else {
        false
    }
}


#[allow(unused)]
fn compare<T: PartialEq>( small_list: &[T], bigger_list: &[T] ) -> bool {
	let small_size: usize = small_list.len();
	let bigger_size: usize = bigger_list.len();
	println!("Lista menor {}, lista maior {}.", small_size, bigger_size);
	//let mut slice = small_list.chunks_exact(small_size);
	let mut result = false;
	//bigger_list.into_iter(|x|

	let mut i: usize = 0;
	let mut j: usize = 0;

	'outer_loop: loop {
		if i > bigger_size - 1 as usize { break }
		if bigger_list[i] == small_list[j] {
			loop {
				if j > small_size - 1 as usize { break }
				if bigger_list[i] == small_list[j] {

					if j == small_size - 1 as usize {
						result = true;
						break;
					} else {
						i +=1;
						//j +=1;
					}
				} else {
					j = 0;
					//i = i + 1;
					continue 'outer_loop;
				}
			}
			i +=1;
			j +=1;

			if j > small_size - 1 as usize { break }
			if i > bigger_size - 1 as usize { break }

		} else {
			j = 0;
			i +=1;
			if i > bigger_list.len() {
				break
			}
		}
	}
	result
}
