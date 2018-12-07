use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Step {
	name: char,
	dependencies: Vec<char>
}

impl PartialOrd for Step {
	fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
        self.name.partial_cmp(&other.name).map(|ord| ord.reverse())
    }
}

impl Ord for Step {
	fn cmp(&self, other: &Step) -> Ordering {
        self.name.cmp(&other.name).reverse()
    }
}

fn parse_steps(lines: &[String]) -> Result<HashMap<char, Step>> {
	let mut map = HashMap::new();

	for line in lines {
		let char1 = line.chars().nth(5).ok_or_else(|| format!("Malformed string: {:#?}", line))?;
		let char2 = line.chars().nth(36).ok_or_else(|| format!("Malformed string: {:#?}", line))?;

		map.entry(char1).or_insert(Step { name: char1, dependencies: Vec::new() });
		map.entry(char2).or_insert(Step { name: char2, dependencies: Vec::new() }).dependencies.push(char1);
	}

	for (_, v) in map.iter_mut() {
		v.dependencies.sort()
	}

	Ok(map)
}

fn work(deps: &HashMap<char, Step>, num_workers: usize, step_duration: &Fn(char) -> i32) -> (String, i32) {
	let mut time_elapsed = 0;
	let mut next_task_avail = BinaryHeap::new();	
	// Note we use negative worker time as a hack, as the collection is a max-heap.
	let mut next_worker_avail: BinaryHeap<i32> = std::iter::repeat(0).take(num_workers).collect();
	let mut finish_at: BTreeMap<i32, _> = BTreeMap::new();
	let mut step_dep_count = HashMap::new();

	let mut done = Vec::new();

	for (c, step) in deps {
		let dep_count = step.dependencies.len();

		if dep_count == 0 {
			next_task_avail.push(step);
		} else {
			step_dep_count.insert(c, dep_count as i32);
		}
	}

	while done.len() < deps.len() {
		if next_task_avail.is_empty() {
			// No tasks can be started. Wait until the next task is finished.
			let next_time = finish_at.keys().nth(0).unwrap();
			while next_worker_avail.peek().unwrap() > &-next_time {
				next_worker_avail.pop();
				next_worker_avail.push(- *next_time);
			}
			time_elapsed = *next_time;
		} else {
			// Wait for the next-available worker and start a task.
			time_elapsed = -next_worker_avail.pop().unwrap();
			let step = next_task_avail.pop().unwrap();

			let duration = step_duration(step.name);
			
			finish_at.entry(time_elapsed + duration).or_insert_with(Vec::new).push(step);
			next_worker_avail.push(-(time_elapsed + duration));
		}

		for step_finished in finish_at.get(&time_elapsed).unwrap_or(&Vec::new()) {
			done.push(step_finished.name);

			for (c, count) in step_dep_count.iter_mut() {
				if deps.get(c).unwrap().dependencies.contains(&step_finished.name) {
					*count -= 1;
				}

				if *count == 0 {
					*count = -1;
					next_task_avail.push(deps.get(c).unwrap());
				}
			}
		}

		finish_at.remove(&time_elapsed);
	}

	(done.iter().collect(), time_elapsed)
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december07.txt")?;
    let deps = parse_steps(&lines)?;

    println!("Part 1: {:?}", work(&deps, 1, &|_| 0));
    println!("Part 2: {:?}", work(&deps, 5, &|c| 60 + 1 + c as i32 - 'A' as i32));
    
    Ok(())
}
