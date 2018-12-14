/// Perform one recipe-making step.
fn step(recipes: &mut Vec<u8>, e1: &mut usize, e2: &mut usize) {
    let sum = recipes[*e1] + recipes[*e2];
    if sum >= 10 {
        recipes.push(sum/10);
        recipes.push(sum%10);
    } else {
        recipes.push(sum);
    }
    *e1 = (*e1 + (recipes[*e1]) as usize + 1) % recipes.len();
    *e2 = (*e2 + (recipes[*e2]) as usize + 1) % recipes.len();
}

/// Run for the given iterations to find the final pattern.
fn run(input: usize) -> Vec<u8> {
    let (mut recipes, mut e1, mut e2) = (vec![3,7], 0, 1);

    while recipes.len() < input as usize + 10 + 2 {
        step(&mut recipes, &mut e1, &mut e2);
    }

    recipes[input as usize..input as usize + 10].to_vec()
}

/// Count iterations until the given pattern occurs.
fn nur(input: &[u8]) -> usize {
    let (mut recipes, mut e1, mut e2) = (vec![3,7], 0, 1);

    loop {
        step(&mut recipes, &mut e1, &mut e2);

        if recipes.len() >= input.len() && recipes[recipes.len() - input.len()..] == *input {
            break recipes.len() - input.len()
        } else if recipes.len() > input.len() && recipes[recipes.len() - input.len() - 1..recipes.len() - 1] == *input {
            break recipes.len() - input.len() - 1   
        }
    }
}

fn main() {
    println!("Part 1: {:?}", run(580741));
    println!("Part 2: {:?}", nur(&vec![5,8,0,7,4,1]));
}
