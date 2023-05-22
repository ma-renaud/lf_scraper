#[allow(unused_imports)]
use anyhow::{anyhow, Result};
#[allow(unused_imports)]
use surrealdb::dbs::{Response, Session};
#[allow(unused_imports)]
use surrealdb::kvs::Datastore;
#[allow(unused_imports)]
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};

mod specie;
mod scaper;
mod db;
mod utils;

use std::sync::Arc;
#[allow(unused_imports)]
use specie::Specie;
#[allow(unused_imports)]
use scaper::{scrape_prices};
use db::DB;


/*TODO:
    [X] Species ORM
    [X] Create a list of existing species
    [X] Scrape a page
    [ ] Check for new species and add them
    [ ] Log all the scraped prices
*/

#[tokio::main]
async fn main() -> Result<()> {
    let ds = Arc::new(Datastore::new("memory").await.unwrap());
    let ses = Session::for_db("my_ns", "my_db");
    let db = DB { ds, ses };

    db.add_specie(59729, "Merisier").await?;
    db.add_specie(59731, "Noyer").await?;


    //let sql = "SELECT * FROM specie";
    //let res = db.execute(sql, None).await?;
    let res = db.get_species().await?;

    let species: Vec<Specie> = res.into_iter().map(|e| e.into()).collect();
    for specie in species.iter() {
        println!("{:?}", specie.id);
    }

    let test = species.into_iter().find(|s| s.id == 59729);

    match test {
        None => println!("Can't find specie"),
        Some(s) => println!("Found: {}", s.id),
    };

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
