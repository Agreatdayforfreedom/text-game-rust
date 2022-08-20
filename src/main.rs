use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

#[derive(Debug)]
struct HistoryData {
    data_type: String,
    tag: String,
    text: String,
    health: i32,
    options: Vec<HistoryData>
}

impl HistoryData {
    fn new(row: StringRecord) -> HistoryData {
        let health: &str = row.get(3).unwrap().trim();
        let health: i32 = health.parse().unwrap_or(0);

        return HistoryData {
            data_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            health: health,
            options: vec![]
        };
    }
}

fn main() {
    let mut health:i32 = 100;
    let mut current_tag = FIRST_TAG.to_string();
    let mut last_record: String = "".to_string();
    let mut histories_data: HashMap<String, HistoryData> = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let data = HistoryData::new(result);
        
        if data.data_type == "SITUACION" {
            let record_tag = data.tag.clone();

            histories_data.insert(record_tag.clone(), data);
            last_record = record_tag;
            
        } else if data.data_type == "OPCION" {
            if let Some(dato) = histories_data.get_mut(&last_record) {
                (*dato).options.push(data);
            }
        }
    }

    let mut dictionary: HashMap<String, String> = HashMap::new();
    dictionary.insert("Apple".to_string(), "Red .fruit".to_string());
    dictionary.insert("PEar".to_string(), "YEllow fruit".to_string());


    //Game loop

    loop {
        println!("tienes {} de vida", health);

        if let Some(dato) = histories_data.get(&current_tag) {
                println!("{}", dato.text);

                for (i, option) in dato.options.iter().enumerate() {
                    println!("[{}] {}", i, option.text);
                }

                let mut selection = String::new();
                std::io::stdin().read_line(&mut selection).unwrap();
                
                let selection = selection.trim().parse().unwrap_or(99);

                if let Some(option_selected) = &dato.options.get(selection) {
                    current_tag = option_selected.tag.to_string();
                } else {
                    print!("Comando no valido");
                }

                health += dato.health;
                println!("");

        }else {
            break;
        }

        if health <= 0 {
            println!("Has perdido");
            break;
        }
    }


}
