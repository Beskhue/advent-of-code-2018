use std::collections::BTreeMap;
mod utils;

fn checksum(ids: &Vec<String>) -> i32 {
    let mut exactly2 = 0;
    let mut exactly3 = 0;
    
    for id in ids {
        let mut letter_count = BTreeMap::new();
        
        for letter in id.chars() {
            let count = letter_count.entry(letter).or_insert(0);
            *count += 1;
        }
        
        if letter_count.iter().any(|(_, &count)| count == 2) {
            exactly2 += 1
        }
        
        if letter_count.iter().any(|(_, &count)| count == 3) {
            exactly3 += 1
        }
    }
    
    exactly2 * exactly3
}

fn find_similar_ids(ids: &Vec<String>) -> Option<(&str, &str)> {
    for id1 in ids {
        for id2 in ids {
            assert!(id1.len() == id2.len());
            let mut diff = 0;
            
            for (char1, char2) in id1.chars().zip(id2.chars()) {
                if char1 != char2 {
                    diff += 1
                }
                
                if diff > 1 {
                    break;
                }
            }
            
            if diff == 1 {
                return Some((id1, id2))
            }   
        };
    }
    
    None
}

fn main() {
    let lines: Vec<String> = utils::lines_from_file("input/december02.txt").unwrap();
    
    println!("Part 1: {:#?}", checksum(&lines));
    println!("Part 2: {:#?}", find_similar_ids(&lines));
}
