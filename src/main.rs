use sublist::{sublist, Comparison};

fn main () {

	match sublist(&[1, 1, 2], &[1, 1, 1, 2]) {
		Comparison::Equal => println!("Iguais"),
		Comparison::Sublist => println!("Sublista"),
		Comparison::Superlist => println!("Superlista"),
		Comparison::Unequal => println!("Diferentes"),
	}
}

