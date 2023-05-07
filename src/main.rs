use std::collections::BTreeMap;
#[allow(unused_imports)]
use anyhow::{anyhow, Result};
use serde::{Serialize, Deserialize};
use scraper::{Element, ElementRef, Html, Selector};
use std::fs;
use yaml_rust::YamlLoader;
use surrealdb::dbs::{Response, Session};
use surrealdb::kvs::Datastore;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};

#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
struct Specie {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u32>,
    name: String,
    price: u16,
    lf_id: u32,
}

type DB = (Datastore, Session);

#[tokio::main]
async fn main() -> Result<()> {
    let db: &DB = &(Datastore::new("memory").await?, Session::for_db("my_ns", "my_db"));
    let (ds, ses) = db;

    let title = "Mon titre";
    let priority = 1;

    let test:(String, Value) = ("title".into(), title.into());

    let data: BTreeMap<String, Value> = [
        ("title".into(), title.into()),
        ("priority".into(), priority.into()),
    ]
        .into();
    let vars: BTreeMap<String, Value> = [("data".into(), data.into())].into();

    
    let sql = "CREATE specie SET name='Merisier', price=433, lf_id=59729";
    let res = ds.execute(sql, ses, None, false).await?;
    let sql = "CREATE specie SET name='Noyer noir', price=1100, lf_id=59731";
    let res = ds.execute(sql, ses, None, false).await?;

    let sql = "SELECT * FROM specie";
    let res = ds.execute(sql, ses, None, false).await?;
    println!("{res:?}");




    // let mut species: Vec<Specie> = Vec::new();
    //
    // scrape_prices(&mut species);
    //
    // println!("{} wood species found.", species.len());
    //
    // for specie in species {
    //     println!("{:?}", specie);
    // }

    Ok(())
}

fn scrape_prices(species: &mut Vec<Specie>) {
    let contents = match fs::read_to_string("langevin.html") {
        Err(e) => panic!("Problem opening html file: {:?}", e),
        Ok(f) => f,
    };

    //let contents = get_page();

    let document = Html::parse_document(&*contents);
    let list_selector = Selector::parse("ol.product-items").unwrap();
    let item_selector = Selector::parse("div.product-item-details").unwrap();

    let list = document.select(&list_selector).next().unwrap();

    for element in list.select(&item_selector) {
        let name = scrape_name(&element);
        let price = scrape_price(&element);
        let lf_id = scrape_id(&element);

        species.push(Specie { id: None, name, price, lf_id });
    }
}

fn scrape_name(element: &ElementRef) -> String {
    let name_selector = Selector::parse("a.product-item-link").unwrap();

    element
        .select(&name_selector)
        .next()
        .unwrap()
        .inner_html()
        .replace(" - bois brut", "")
        .replace(" - Bois brut", "")
}

fn scrape_price(element: &ElementRef) -> u16 {
    let price_selector = Selector::parse("span.price").unwrap();

    let price_html = element.select(&price_selector).next().unwrap();
    let formatted_price = price_html
        .inner_html()
        .replace("&nbsp;$", "")
        .replace(",", "");

    match formatted_price.parse::<u16>() {
        Err(e) => panic!("Problem converting price string: {:?}", e),
        Ok(p) => p,
    }
}

fn scrape_id(element: &ElementRef) -> u32 {
    let id_selector = Selector::parse("div.price-final_price").unwrap();

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

    match id_str.parse::<u32>() {
        Err(e) => panic!("Problem converting ID string: {:?}", e),
        Ok(id) => id,
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
