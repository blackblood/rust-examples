struct Node {
	data: Option<u32>,
	next_node: Option<Box<Node>>
}

impl Node {
	fn new(val: Option<u32>, next: Option<Node>) -> Node {
		match val {
			Some(v) => match next {
				Some(n) => Node { data: Some(v), next_node: Some(Box::new(n)) },
				None => Node { data: Some(v), next_node: None }
			},
			None => match next {
				Some(n) => Node { data: None, next_node: Some(Box::new(n)) },
				None => Node { data: None, next_node: None }
			}
		}
	}
}

fn main() {
	let n1: Node = Node::new(Some(2), None);
	let n2: Node = Node::new(Some(3), Some(n1));
	let n3: Node = Node::new(Some(4), Some(n2));
	let n4: Node = Node::new(Some(4), Some(n3));

	let mut n: Node = n4;
	loop {
		if let Some(val) = n.data {
			println!("{}", val);
		}
		if let Some(nxt) = n.next_node {
			n = *nxt;
		} else {
			break;
		}
	}
}