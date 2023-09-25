use std::collections::HashMap;

struct CacheForMoves {
	set_of_moves: Vec<usize>,
	cache: HashMap<usize, Vec<Vec<usize>>>
}

impl CacheForMoves {
	fn new(set_of_moves: Vec<usize>) -> CacheForMoves {
		CacheForMoves {
			set_of_moves: set_of_moves,
			cache: HashMap::new()
		}
	}

	fn get_for_n(&self, n: usize) -> Option<&Vec<Vec<usize>>> {
		self.cache.get(&n)
	}

	fn insert_for_n(&mut self, n: usize, value: Vec<Vec<usize>>) {
		self.cache.insert(n, value);
	}
}

fn stairs(cache: &mut CacheForMoves, n: usize) -> &Vec<Vec<usize>> {
	return match cache.get_for_n(n) {
		Some(result) => result,
		None => stairs(cache, n - 1)
	}
}

fn main() {
	let mut cache = CacheForMoves::new(vec![1, 2]);
	cache.insert_for_n(1, vec![]);
	let result = stairs(&mut cache, 4);
	println!("Found {} possible solutions: ", result.len());
	for solution in result {
		println!("{:?}", solution);
	}
}