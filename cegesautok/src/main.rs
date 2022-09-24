#![allow(unused)]

use std::collections::{BTreeMap};
use std::io::{BufReader, BufWriter, Write, BufRead, self};
use std::fs::File;

struct Record {
    day: u8,
    time: String,
    plate_number: String,
    employee_id: u16,
    odometer_state: u32,
    direction: Direction
}

#[derive(PartialEq)]
enum Direction {
    TakeOut,
    Return
}


fn parse_text_file() -> Vec<Record> {
    let mut records: Vec<Record> = Vec::new();
    let input_file = File::open("autok.txt").expect("Error opening autok.txt!");

    let reader = BufReader::new(input_file);

    for line in reader.lines() {
        let line = line.expect("Error reading line from autok.txt").trim().to_owned();
        if line.is_empty() {continue}

        let data: Vec<&str> = line.split(" ").collect();

        records.push(Record {
            day: data[0].parse().expect("Failed parsing day."),
            time: data[1].to_string(),
            plate_number: data[2].to_string(),
            employee_id: data[3].parse().expect("Failed parsing employee_id."),
            odometer_state: data[4].parse().expect("Failed parsing odometer state."),
            direction: match data[5] {
                "0" => Direction::TakeOut,
                "1" => Direction::Return,

                _ => Direction::TakeOut
            }
        });
    }

    return records;
}

fn task_1(records: &Vec<Record>) {
    let mut last_car = records.iter().rev().find(|x| x.direction == Direction::TakeOut).expect("Couldn't find last car.");
    println!("2. feladat\n{}. nap rendszám: {}", last_car.day, last_car.plate_number);
}

fn task_2(records: &Vec<Record>) {
    print!("Nap: ");
    io::stdout().flush().expect("Couldn't flush stdout buffer.");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input);

    let day_parse: Result<u8, _> = input.trim_end().parse();
    let mut day: u8 = 0;

    match day_parse {
        Ok(d) => {day = d}
        Err(_) => {println!("Hibás bemenet!");return}
    }

    println!("Forgalom a(z) {}. napon:", day);

    for record in records {
        if record.day > day {break;}

        if record.day == day {
            println!("{} {} {} {}", record.time, record.plate_number, record.employee_id,
            match record.direction{
                Direction::TakeOut => "ki",
                Direction::Return => "be"
            });
        }
    }
}

fn task_3(records: &Vec<Record>) {
    let mut i = 0;

    for record in records {
        i += match record.direction {
            Direction::TakeOut => 1,
            Direction::Return => -1
        };
    }

    println!("A hónap végén {} autót nem hoztak vissza.", i)
}

fn task_4(records: &Vec<Record>) {
    let mut stat: BTreeMap<String, u32> = BTreeMap::new();
    let mut last_state: BTreeMap<String, u32> = BTreeMap::new();

    for record in records {

        let odometer = stat.get(&record.plate_number).unwrap_or(&0);

        if record.direction == Direction::Return {
            let last_odometer_state = last_state.get(&record.plate_number).expect("M");

            stat.insert(record.plate_number.clone(), (record.odometer_state - last_odometer_state) +odometer);
        } else {
            last_state.insert(record.plate_number.clone(), record.odometer_state);
        }
        
    }

    for (key, value) in stat {
        println!("{} {} km", key, value)
    }
}

fn task_5(records: &Vec<Record>) {
    let mut last_record: BTreeMap<String, u32> = BTreeMap::new();
    let mut trips: Vec<(u16, u32)> = Vec::new();

    for record in records {
        if record.direction == Direction::TakeOut {
            last_record.insert(record.plate_number.clone(), record.odometer_state);
        } else {
            let last_odometer = last_record.get(&record.plate_number).expect("last_record doesn't contain current returning vehicle.");
            trips.push((record.employee_id, record.odometer_state-last_odometer));
        }
    }

    trips.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Comparison failed"));

    let longest = trips.last().expect("Trips is empty.");
    println!("Leghosszabb út: {} km, személy: {}", longest.1, longest.0);
}

fn task_6(records: &Vec<Record>) {
    print!("7. feladat\nRendszám: ");
    io::stdout().flush();

    let mut plate_number: String = String::new();
    io::stdin().read_line(&mut plate_number);

    plate_number = plate_number.trim().to_owned();

    match records.iter().any(|x| x.plate_number == plate_number) {
        true => {}
        false => {
            println!("Nem létezik ilyen rendszám!");
            return;
        }
    }

    let mut trips: Vec<[&Record; 2]> = Vec::new();
    let mut take_out_record: Option<&Record> = None;

    for record in records {
        if record.plate_number != plate_number {continue}
        
        if record.direction == Direction::TakeOut {
            take_out_record = Some(record);
        } else {
            let t = take_out_record.expect("Vehicle takeout record does not appear before return.");

            trips.push([t, record]);

            take_out_record = None;
        }
        
    }
    
    let output_file = File::create(format!("{}_menetlevel.txt", plate_number)).expect("Failed creating output file.");
    let mut writer = BufWriter::new(output_file);

    for trip in trips {
        let line = format!("{}\t{}. {}\t{} km\t{}. {}\t{} km\n",
        trip[0].employee_id,
        trip[0].day,
        trip[0].time,
        trip[0].odometer_state,
        trip[1].day,
        trip[1].time,
        trip[1].odometer_state
        );

        writer.write_all(line.as_bytes());
    }

    match take_out_record {
        Some(r) => {
            let line = format!("{}\t{}. {}\t{} km\n",
            r.employee_id,
            r.day,
            r.time,
            r.odometer_state,
            );

            writer.write_all(line.as_bytes());
        }

        None => {}
    }
}

fn main() {
    let records = parse_text_file();

    task_1(&records);
    task_2(&records);
    task_3(&records);
    task_4(&records);
    task_5(&records);
    task_6(&records);
}
