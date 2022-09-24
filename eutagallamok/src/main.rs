use std::io::{BufReader, BufRead};
use std::fs::File;

use std::collections::HashMap;


struct Tagallam {
    name: String,
    join_date: String
}



fn main(){
    let file = File::open("EUcsatlakozas.txt").expect("EUcsatlakozas.txt nem letezik!");
    let reader = BufReader::new(file);
    let mut tagallamok: Vec<Tagallam> = Vec::new();


    for line in reader.lines() {
        let line_text = line.expect("Line could not be read!");

        let items = line_text.split(";").collect::<Vec<&str>>();
        
        let t = Tagallam {
            name: String::from(items[0]),
            join_date: String::from(items[1])
        };

        tagallamok.push(t);
    }
    task_1(&tagallamok);
    task_2(&tagallamok);
    task_3(&tagallamok);
    task_4(&tagallamok);
    task_5(&mut tagallamok);
    task_6(&tagallamok);
}


fn task_1(tagallamok: &Vec<Tagallam> ) {
    println!("3. feladat: EU tagállamainak száma: {} db", tagallamok.len());
}

fn task_2(tagallamok: &Vec<Tagallam>) {
    let mut joined_in_2007 = 0;
    
    for tagallam in tagallamok {
        if &tagallam.join_date[0..4] == "2007" {
            joined_in_2007 += 1;
        }
    }

    println!("4. feladat: 2007-ben {} ország csatlakozott.", joined_in_2007);
}

fn task_3(tagallamok: &Vec<Tagallam>) {
    let mut hungary_join_date = "";
    
    for tagallam in tagallamok {
        if tagallam.name == "Magyarország" {
            hungary_join_date = &tagallam.join_date;
        }
    }
    
    println!("5. feladat: Magyarország csatlakozásának dátuma: {}", hungary_join_date);
}

fn task_4(tagallamok: &Vec<Tagallam>) {
    let mut joined_in_may = false;

    for tagallam in tagallamok {
        if&tagallam.join_date[5..7] == "05" {
            joined_in_may = true;
            break;
        }
    }

    let txt = if joined_in_may {"volt"} else {"nem volt"};

    println!("6. feladat: Májusban {} csatlakozás!", txt);
}

fn task_5(tagallamok: &mut Vec<Tagallam>) {
    let mut tcpy: Vec<Tagallam> = Vec::new();

    for t in tagallamok {
        let tgcpy = Tagallam {
            name: t.name.clone(),
            join_date: t.join_date.clone()
        };

        tcpy.push(tgcpy);
    }
    tcpy.sort_by(|a, b| a.join_date.to_lowercase().cmp(&b.join_date.to_lowercase()));

    println!("7. feladat: Legutoljára csatlakozott ország: {}", tcpy.last().expect("Data vector is empty!").name);
}

fn task_6(tagallamok: &Vec<Tagallam>) {
    println!("8. feladat: Statisztika");

    let mut stat: HashMap<String, u8> = HashMap::new();

    for t in tagallamok {

            stat.insert(t.join_date[..4].to_string(), stat.get(&t.join_date[..4]).unwrap_or(&0).to_owned()+1 );

    }

    for k in stat.keys() {
        println!("\t{} - {} ország", k, stat.get(k).unwrap());
    }
}