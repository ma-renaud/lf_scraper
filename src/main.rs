use scraper::{Html, Selector};
#[allow(unused_imports)]
use std::fs;
use yaml_rust::YamlLoader;

#[derive(Debug)]
struct Specie {
    name: String,
    price: u16,
    id: u32,
}

fn main() {
    let contents = match fs::read_to_string("langevin.html") {
        Err(e) => panic!("Problem opening html file: {:?}", e),
        Ok(f) => f,
    };

    //let contents = get_page();

    let mut species: Vec<Specie> = Vec::new();

    let document = Html::parse_document(&*contents);
    let list_selector = Selector::parse("ol.product-items").unwrap();
    let item_selector = Selector::parse("div.product-item-details").unwrap();
    let name_selector = Selector::parse("a.product-item-link").unwrap();
    let price_selector = Selector::parse("span.price").unwrap();
    let id_selector = Selector::parse("div.price-final_price").unwrap();
    let list = document.select(&list_selector).next().unwrap();

    for element in list.select(&item_selector) {
        let name = element
            .select(&name_selector)
            .next()
            .unwrap()
            .inner_html()
            .replace(" - bois brut", "")
            .replace(" - Bois brut", "");

        let price_html = element.select(&price_selector).next().unwrap();
        let formatted_price = price_html
            .inner_html()
            .replace("&nbsp;$", "")
            .replace(",", "");
        let price = match formatted_price.parse::<u16>() {
            Err(e) => panic!("Problem converting price string: {:?}", e),
            Ok(p) => p,
        };

        let id_attr = element
            .select(&id_selector)
            .next()
            .unwrap()
            .value()
            .attr("data-product-id");

        let id_str = match id_attr {
            None => panic!("Can't read ID attribute"),
            Some(id) => id,
        };

        let id = match id_str.parse::<u32>() {
            Err(e) => panic!("Problem converting ID string: {:?}", e),
            Ok(id) => id,
        };

        species.push(Specie { name, price, id });
    }

    println!("{} wood species found.", species.len());

    for specie in species {
        println!("{:?}", specie);
    }
}

fn load_api_key() -> String {
    let key_file = match fs::read_to_string("key.yaml") {
        Err(e) => panic!("Problem opening key file: {:?}", e),
        Ok(f) => f,
    };

    let docs = YamlLoader::load_from_str(&*key_file).unwrap();
    if docs.is_empty() {
        panic!("Key file is empty");
    }
    let doc = &docs[0];

    match doc["key"].as_str() {
        None => panic!("Can't read key in key.yaml"),
        Some(key) => return key.to_string(),
    }
}

#[allow(unused)]
fn get_page() -> String {
    let api_key: &str = &*load_api_key();
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let response = client
        .get(format!("https://app.crawlio.net/api/v1?api_key={}&url=https%3A%2F%2Fwww.langevinforest.com%2Ffr%2Fbois%2Fbois-brut%3Fproduct_list_limit%3Dall&proxy_tier=standard", api_key))
        // confirm the request using send()
        .send()
        // the rest is the same!
        .unwrap()
        .text()
        .unwrap();

    return response;
}
