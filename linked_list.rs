#[derive(Clone)]
struct Node {
	data: u32,
	next: Option<Box<Node>>
}

fn main() {
	let n = Node { data: 1, next: None };
	let n2 = Node { data: 2, next: Some(Box::new(n)) };
	let n3 = Node { data: 3, next: Some(Box::new(n2)) };
	let mut current_node = Some(Box::new(Node { data: 4, next: Some(Box::new(n3)) }));
	
	loop {
		match current_node {
			Some(node) => {
				println!("{}", node.data);
				current_node = node.next;
			},
			None => break
		}
	}
}