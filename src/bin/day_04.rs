#![recursion_limit = "1024"]

extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate regex;

use chrono::NaiveDateTime;
use chrono::Timelike;
use regex::Regex;
use std::collections::HashMap;
use std::io::Read;

mod errors {
    error_chain!{}
}

use errors::*;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let events = parse_guard_shifts(&data).unwrap();

    println!("part_1: {}", part_1(&events));
    println!("part_2: {}", part_2(&events));
}

fn part_1(events: &Vec<GuardShiftEvent>) -> usize {
    let guard_minutes = collect_guard_minutes(events);

    let (sleepiest_guard_id, _sleep_total) = guard_minutes
        .iter()
        .map(|(guard_id, counts)| (guard_id, counts.iter().sum::<usize>()))
        .max_by_key(|(_guard_id, sum)| sum.clone())
        .unwrap();

    let sleepiest_guard_minutes = guard_minutes.get(sleepiest_guard_id).unwrap();
    let sleepiest_minute = (0..sleepiest_guard_minutes.len())
        .max_by_key(|&k| sleepiest_guard_minutes[k])
        .unwrap();
    return sleepiest_guard_id * sleepiest_minute;
}

fn part_2(events: &Vec<GuardShiftEvent>) -> usize {
    let guard_minutes = collect_guard_minutes(events);

    let (guard_id, _, minute) = guard_minutes
        .iter()
        .map(|(guard_id, counts)| {
            (
                guard_id,
                counts,
                (0..60_usize).max_by_key(|&i| counts[i]).unwrap(),
            )
        }).max_by_key(|(_guard_id, counts, sleepiest_minute)| counts[*sleepiest_minute])
        .unwrap();

    return guard_id * minute;
}

fn collect_guard_minutes(events: &Vec<GuardShiftEvent>) -> HashMap<usize, Vec<usize>> {
    use GuardShiftEventType::*;
    let mut sleep_start_option: Option<NaiveDateTime> = None;
    let mut guard_minutes = HashMap::new();
    let mut guard_id = 0;
    for event in events.iter() {
        match event.event_type {
            FallAsleep => {
                sleep_start_option = Some(event.timestamp);
            }
            WakeUp | BeginShift(_) => {
                if let Some(sleep_start) = sleep_start_option {
                    let mut minutes = guard_minutes
                        .entry(guard_id)
                        .or_insert(vec![0; 60]);
                    let mut minute_iter = sleep_start;
                    while minute_iter < event.timestamp {
                        minutes[minute_iter.time().minute() as usize] += 1;
                        minute_iter += chrono::Duration::minutes(1);
                    }
                }
                guard_id = match event.event_type {
                    BeginShift(new_guard_id) => new_guard_id,
                    _ => guard_id,
                };
                sleep_start_option = None;
            }
        };
    }
    return guard_minutes;
}

#[derive(Debug)]
enum GuardShiftEventType {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct GuardShiftEvent {
    timestamp: NaiveDateTime,
    event_type: GuardShiftEventType,
}

fn parse_guard_shifts(data: &str) -> Result<Vec<GuardShiftEvent>> {
    let re = Regex::new(r"^\[(?P<ts>\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (?P<event>(Guard #(?P<id>\d+))|(falls)|(wakes)).*$").unwrap();
    let mut events = Vec::new();
    for line in data.lines() {
        let caps = re
            .captures(line)
            .chain_err(|| "Unable to match against regular expression")?;
        let timestamp_str = caps.name("ts").chain_err(|| "No timestamp")?.as_str();
        let timestamp = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M")
            .or(Err("Invalid timestamp"))?;
        let event_str = caps
            .name("event")
            .chain_err(|| "Invalid event format")?
            .as_str();
        let event_type: GuardShiftEventType;
        if event_str.starts_with("Guard") {
            let guard_id = caps
                .name("id")
                .chain_err(|| "id not present")?
                .as_str()
                .parse()
                .chain_err(|| "Unable to parse guard_id")?;
            event_type = GuardShiftEventType::BeginShift(guard_id);
        } else if event_str.starts_with("falls") {
            event_type = GuardShiftEventType::FallAsleep;
        } else if event_str.starts_with("wakes") {
            event_type = GuardShiftEventType::WakeUp;
        } else {
            bail!("This is not supposed to be possible");
        }
        events.push(GuardShiftEvent {
            timestamp: timestamp,
            event_type: event_type,
        });
    }
    events.sort_unstable_by_key(|a| a.timestamp);
    return Ok(events);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_1() {
        use parse_guard_shifts;
        use part_1;
        let data = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let events = parse_guard_shifts(data).unwrap();
        assert_eq!(part_1(&events), 240);
    }

    #[test]
    fn test_part_2() {
        use parse_guard_shifts;
        use part_2;
        let data = r"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let events = parse_guard_shifts(data).unwrap();
        assert_eq!(part_2(&events), 4455);
    }
}
