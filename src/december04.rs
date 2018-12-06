extern crate regex;
use std::collections::HashMap;
mod utils;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

#[derive(Debug)]
enum Event {
    Start(i32),
    Sleep,
    Wake
}

/// Parse events into a vector of time+Event tuples. Times are a direct integer
/// representation of [year][month][day][hour][minute]. For example,
/// "2018-12-25 23:57" is 201812252357
fn parse_events(lines: &[String]) -> Result<Vec<(i64, Event)>> {
    let re = regex::Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:(?P<minute>\d{2})\] ((?P<sleep>falls asleep)|(?P<wake>wakes up)|(Guard #(?P<guard>\d+) begins shift))$")?;
    
    let mut lines: Vec<String> = lines.iter().cloned().collect();
    lines.sort();

    let v = lines
        .iter()
        .map(|s| {
            let captures = re.captures(s).ok_or_else(|| format!("String {} does not match", s))?;

            let minute = captures.name("minute").ok_or("Parse err")?.as_str();
            let sleep = captures.name("sleep");
            let wake = captures.name("wake");
            let guard = captures.name("guard");
            
            let event = match (guard, sleep, wake) {
                (Some(n), None, None) => Ok(Event::Start(n.as_str().parse::<i32>()?)),
                (None, Some(_), None) => Ok(Event::Sleep),
                (None, None, Some(_)) => Ok(Event::Wake),
                _ => Err("Parse failure")
            };
            
            Ok((minute.parse::<i64>()?, event?))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(v)
}

/// Split a vector of times and Events into a map of such vectors, with guard
/// shift start events removed.
fn events_by_guard(events: Vec<(i64, Event)>) -> HashMap<i32, Vec<(i64, Event)>> {
    let mut map = HashMap::new();
    let mut current_guard = -1;
    
    for (time, event) in events {
        match event {
            Event::Start(n) => {
                current_guard = n
            }
            _ => {
                map.entry(current_guard).or_insert_with(Vec::new).push((time, event))
            }
        }
    }
    
    map
}

fn part1(guard_events: &HashMap<i32, Vec<(i64, Event)>>) -> Result<i64> {
    let mut sleep_time: Vec<(i32, i64)> = Vec::new();
    
    for (guard, events) in guard_events {
        sleep_time.push((*guard, 
            events.iter()
                .enumerate()
                .filter(|(idx, _)| idx%2 == 1)
                .map(|(_, (t, _))| t)
                .sum::<i64>()
            - events.iter()
                .enumerate()
                .filter(|(idx, _)| idx%2 == 0)
                .map(|(_, (t, _))| t)
                .sum::<i64>()
        ))
    }
    
    let (guard, _) = *sleep_time.iter().max_by_key(|(_, t)| t).ok_or("No events")?;
    let mut sleep_minute_count: HashMap<i64, i32> = HashMap::new();
    let mut sleep_start_minute = 0;
    
    for (idx, (t, _)) in guard_events[&guard].iter().enumerate() {
        if idx % 2 == 0 {
            sleep_start_minute = *t
        } else {
            let to = *t;
            
            for minute in sleep_start_minute..to {
                let count = sleep_minute_count.entry(minute).or_insert(0);
                *count += 1;
            }
        }
    }
    
    let (minute, _) = sleep_minute_count.iter().max_by_key(|(_, t)| *t).ok_or("No events")?;
    
    Ok(i64::from(guard) * minute)
}

fn part2(guard_events: &HashMap<i32, Vec<(i64, Event)>>) -> Result<i64> {
    let mut max_guard = 0;
    let mut max_minute = 0;
    let mut max_count = 0;
    
    for (guard, events) in guard_events {
        let mut sleep_minute_count: HashMap<i64, i32> = HashMap::new();
        let mut sleep_start_minute = 0;
        for (idx, (t, _)) in events.iter().enumerate() {
            if idx % 2 == 0 {
                sleep_start_minute = *t
            } else {
                let to = *t;
                
                for minute in sleep_start_minute..to {
                    let count = sleep_minute_count.entry(minute).or_insert(0);
                    *count += 1;
                }
            }
        }
        
        let (minute, count) = sleep_minute_count.iter().max_by_key(|(_, t)| *t).ok_or("No events")?;
        
        if *count > max_count {
            max_guard = *guard;
            max_minute = *minute;
            max_count = *count;
        }
    }
    
    Ok(i64::from(max_guard) * max_minute)
}

fn main() -> Result<()> {
    let lines = utils::lines_from_file("input/december04.txt")?;
    let events = events_by_guard(parse_events(&lines)?);

    println!("Part 1: {:#?}", part1(&events)?);
    println!("Part 2: {:#?}", part2(&events)?);
    
    Ok(())
}
