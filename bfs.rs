use std::collections::VecDeque;

#[derive(Clone)]
struct Node {
	data: u32,
	connections: Vec<Node>
}

fn add_connections(mut node: Node, level: u32) -> Node {
	if level == 3 {
		return node;
	}
	for i in level..(level + 6) {
		let curr_node = Node { data: i, connections: Vec::new() };
		node.connections.push(add_connections(curr_node, level + 1));
	}
	return node;
}

fn bfs(mut queue: VecDeque<&Node>) {
	let el_opt = queue.pop_front();
	match el_opt {
		Some(el) => {
			for l in el.connections.iter() {
				queue.push_back(l);
			}
			bfs(queue);
		},
		None => return
	}
}

fn main() {
	let n = Node { data: 2, connections: Vec::new() };
	let n = add_connections(n, 1);
	
	let mut queue: VecDeque<&Node> = VecDeque::new();
	queue.push_back(&n);
	bfs(queue);
}