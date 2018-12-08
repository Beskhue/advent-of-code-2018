mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn sum1(tree: &[u32]) -> (&[u32], u32) {
	let mut tree = tree;
	let mut sum_metadata = 0;
    let num_nodes = tree[0];
    let num_metadata = tree[1];

    tree = &tree[2..];
    for _ in 0..num_nodes {
    	let (new_tree, sum) = sum1(tree);
    	tree = new_tree;
    	sum_metadata += sum;
    }

    for i in 0..num_metadata {
    	sum_metadata += tree[i as usize];
    }

    (&tree[num_metadata as usize..], sum_metadata)
}

fn sum2(tree: &[u32]) -> (&[u32], u32) {
	let mut tree = tree;
	let mut sum_metadata = 0;
	let mut child_val = Vec::new();
    let num_nodes = tree[0];
    let num_metadata = tree[1];

    tree = &tree[2..];
    for _ in 0..num_nodes {
    	let (new_tree, sum) = sum2(tree);
    	tree = new_tree;
    	child_val.push(sum);
    }

    for i in 0..num_metadata {
    	if num_nodes == 0 {
    		sum_metadata += tree[i as usize];
    	} else {
			let idx = tree[i as usize] - 1;
			if (idx as usize) < child_val.len() {
				sum_metadata += child_val[idx as usize];
			}

    	}
    }

    (&tree[num_metadata as usize..], sum_metadata)
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december08.txt")?;
    let tree: Vec<u32> = lines[0].split(" ").map(|c| c.parse::<u32>().unwrap()).collect();

    println!("Part 1: {:?}", sum1(&tree));
    println!("Part 2: {:?}", sum2(&tree));
    
    Ok(())
}
