use std::collections::{BTreeMap};
use std::io;
use std::io::{BufReader, BufRead, Write, BufWriter};
use std::fs::File;

#[derive(PartialEq)]
#[derive(Clone)]
enum TaxBracket {
    A,
    B,
    C
}

impl TaxBracket {
    pub fn from(c: char) -> Result<TaxBracket, ()> {
        match c {
            'A' => Ok(TaxBracket::A),
            'B' => Ok(TaxBracket::B),
            'C' => Ok(TaxBracket::C),
            _ => Err(())
        }
    }
    
}


struct Building {
    tax_number: u32,
    street: String,
    street_number: String,
    tax_bracket: TaxBracket,
    plot_size: u16,
    tax_amount: u64
}

fn get_tax_prices(reader: &mut BufReader<File>) -> [u16; 3] {
    let mut first_line: String = String::new();
    reader.read_line(&mut first_line).expect("Error reading first line.");

    let mut tax_prices: [u16; 3] = [0; 3];

    for (index, price) in first_line.split(" ").enumerate() {
        tax_prices[index] = price.trim_end().parse().expect("Parsing price into uint16 is unsuccessful.");
    } 

    return tax_prices;
}

fn parse_text_file(reader: &mut BufReader<File>) -> Vec<Building> {
    let mut buildings: Vec<Building> = Vec::new();


    for line in reader.lines() {
        let line = line.expect("Error reading line.");
        if line.is_empty() { continue; }

        let data: Vec<&str> = line.split(" ").collect();

        buildings.push(Building {
            tax_number: data[0].parse().expect("Failed parsing tax number."),
            street: data[1].to_string(),
            street_number: data[2].to_string(),
            tax_bracket: TaxBracket::from(data[3].chars().nth(0).unwrap()).expect("Invalid tax bracket specified."),
            plot_size: data[4].parse().expect("Failed parsing plot size."),
            tax_amount: 0
        });

    }

    return buildings;
}

fn calculate_tax(buildings: &mut Vec<Building>, tax_rates: &[u16; 3]) {
    for building in buildings {
        building.tax_amount = ado(&building.tax_bracket, &building.plot_size, &tax_rates);
    }
}

fn ado(tax_bracket: &TaxBracket, plot_size: &u16, tax_rates: &[u16; 3]) -> u64{
    let tax_amount: u64;

    match tax_bracket {
        TaxBracket::A => {tax_amount = u64::from(plot_size.to_owned()) * u64::from(tax_rates[0])}
        TaxBracket::B => {tax_amount = u64::from(plot_size.to_owned()) * u64::from(tax_rates[1])}
        TaxBracket::C => {tax_amount = u64::from(plot_size.to_owned()) * u64::from(tax_rates[2])}
    }

    if tax_amount < 10000 {return 0}
    
    return tax_amount;
}

fn task_1(buildings: &Vec<Building>) {
    println!("2. feladat. A mintában {} telek szerepel.", buildings.len());
}

fn task_2(buildings: &Vec<Building>) {
    let exit_msg = "Nem szerepel az adatállományban.";

    print!("3. feladat. Egy tulajdonos adószáma: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut tax_number = String::new();

    io::stdin().read_line(&mut tax_number).expect("Failed to read stdin");

    let tax_number_parse: Result<u32, _> = tax_number.trim_end().parse();
    let tax_number: u32;
    match tax_number_parse {
        Ok(t) => {
            tax_number = t;
        },

        Err(_) => {
            println!("{exit_msg}");
            return;
        }
    }

    let search = buildings.iter().find(|&x| x.tax_number == tax_number);
    match search {
        None => { println!("{exit_msg}"); return;},
        Some(b) => {
            println!("{} utca {}", b.street, b.street_number);
        }
    }


    

}

fn task_3(buildings: &Vec<Building>) {
    let mut taxes: [u64; 3] = [0; 3];
    let mut building_count: [u16; 3] = [0; 3];

    for building in buildings {

        match building.tax_bracket  {
            TaxBracket::A => {taxes[0] += building.tax_amount; building_count[0] +=1},
            TaxBracket::B => {taxes[1] += building.tax_amount; building_count[1] +=1},
            TaxBracket::C => {taxes[2] += building.tax_amount; building_count[2] +=1},
        }

    }
    println!("5. feladat");

    for (ind, val) in ['A', 'B', 'C'].iter().enumerate() {
        println!("{} sávba {} telek esik, az adó {} Ft.", val, building_count[ind], taxes[ind]);
    }
}

fn task_4(buildings: &Vec<Building>) {
    let mut brackets_street: BTreeMap<String, Vec<TaxBracket>> = BTreeMap::new();

    for building in buildings {
        if !brackets_street.contains_key(&building.street) {
            brackets_street.insert(building.street.clone(), Vec::new());
        }

        let mut brackets = brackets_street.get(&building.street).expect("Error during HashMap retrieval of street.").clone();

        if !brackets.contains(&building.tax_bracket) {
            brackets.push(building.tax_bracket.clone());
        }

        brackets_street.insert(building.street.clone(), brackets);
        
    }

    for (street, brackets) in brackets_street.iter() {
        if brackets.len() > 1 {
            println!("{street}");
        }
    }
}

fn task_5(buildings: &Vec<Building>) {
    let mut taxes_per_owner: BTreeMap<u32, u64> = BTreeMap::new();
    let default_value: u64 = 0;
    for building in buildings {
        taxes_per_owner.insert(building.tax_number.clone(), taxes_per_owner.get(&building.tax_number).unwrap_or(&default_value)+building.tax_amount);
    }
    
    
    let output_file = File::create("fizetendo.txt").expect("fizetendo.txt Already exists!");
    let mut writer = BufWriter::new(output_file);

    for (tax_number, amount) in taxes_per_owner {
        writer.write_all(format!("{} {}\n", tax_number, amount).as_bytes()).expect("Error while trying to write to output file.");
    }
}

fn main() {
    let input_file = File::open("utca.txt").expect("utca.txt nem letezik!");
    let mut reader = BufReader::new(input_file);
    
    let tax_rates = get_tax_prices(&mut reader);

    let mut buildings = parse_text_file(&mut reader);

    calculate_tax(&mut buildings, &tax_rates);

    task_1(&buildings);
    task_2(&buildings);

    task_3(&buildings);

    task_4(&buildings);

    task_5(&buildings);
    
}

