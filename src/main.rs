use std::{fs, path::Path};

use scraper::Html;
use serde::Serialize;

#[derive(Serialize)]
struct Champion {
    id: String,
    name: String,
}

fn main() {
    let data_path = Path::new("data");
    if !data_path.exists() {
        fs::create_dir(data_path).expect("Unable to create data directory");
    }

    let roles = ["top", "jungle", "mid", "adc", "support"];
    for role in roles {
        println!("Getting data for {}", role);

        let data = get_data(role).expect("Error getting data");
        let champions = get_champions(data);

        let serialized = serde_json::to_string(&champions).unwrap();

        let path = format!("data/{}.json", role);
        fs::write(&path, serialized).expect("Unable to write file");

        println!("Data for {} saved in file {}", role, path);
    }
}

fn get_data(position: &str) -> reqwest::Result<Html> {
    let res = reqwest::blocking::get(
        format!("https://www.op.gg/champions?region=global&tier=emerald_plus&position={}", position)
    );
    let content = res
        .expect("Error sending request")
        .text()
        .expect("Error reading response");

    Ok(scraper::html::Html::parse_document(&content))
}

fn get_champions(document: Html) -> Vec<Champion> {
    let champions_selector = scraper::Selector::parse("td.css-1im1udv.e1u05mw02").unwrap();
    let champions = document.select(&champions_selector);

    let mut out_champions: Vec<Champion> = Vec::new();

    for champion in champions {
        let url = champion
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|e| e.value().attr("href"))
            .unwrap();

        let id = url
            .split("/")
            .collect::<Vec<&str>>()
            .get(2)
            .unwrap()
            .to_string();

        let name = champion
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap();

        out_champions.push(Champion { id, name });
    }

    out_champions
}
