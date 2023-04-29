use scraper::{Html, Selector};
use yaml_rust::YamlLoader;
#[allow(unused_imports)]
use std::fs;

fn main() {
    let contents = match fs::read_to_string("langevin.html") {
        Err(e) => panic!("Problem opening html file: {:?}", e),
        Ok(f) => f,
    };

    //let contents = get_page();

    let document = Html::parse_document(&*contents);
    let list_selector = Selector::parse("ol.product-items").unwrap();
    let item_selector = Selector::parse("div.product-item-details").unwrap();
    let name_selector = Selector::parse("a.product-item-link").unwrap();
    let list = document.select(&list_selector).next().unwrap();

    for element in list.select(&item_selector) {
        let name = element.select(&name_selector).next().unwrap();
        println!("{}", name.inner_html());
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
    let api_key:&str = &*load_api_key();
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
