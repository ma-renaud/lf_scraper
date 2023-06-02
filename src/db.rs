use crate::models::{Log, Specie};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;

pub fn get_id_from_thing(thing: &Thing) -> Option<i64> {
    match thing.id {
        Id::Number(id) => Some(id),
        Id::String(_) => None,
        Id::Array(_) => None,
        Id::Object(_) => None,
    }
}

pub struct DbRemote {
    pub client: Surreal<Client>,
}

impl DbRemote {
    pub async fn new(
        address: &str,
        username: &str,
        password: &str,
        namespace: &str,
        database: &str,
    ) -> surrealdb::Result<Self> {
        let client = Surreal::new::<Ws>(address).await?;

        client.signin(Root { username, password }).await?;

        // Select a specific namespace / database
        client.use_ns(namespace).use_db(database).await?;

        Ok(Self { client })
    }

    pub async fn get_species(&self) -> Result<Vec<Specie>, surrealdb::Error> {
        self.client.select("specie").await
    }

    pub async fn get_logs(&self) -> Result<Vec<Log>, surrealdb::Error> {
        self.client
            .query(" SELECT *, specie.* FROM log;")
            .await?
            .take(0)
    }

    pub async fn add_specie(&self, lf_id: i64, name: &str) -> Result<(), surrealdb::Error> {
        let specie: Result<Specie, surrealdb::Error> = self
            .client
            .create("specie")
            .content(Specie {
                id: Thing {
                    tb: String::from("specie"),
                    id: Id::from(lf_id),
                },
                name: String::from(name),
            })
            .await;

        match specie {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn add_log(&self, id: i64, price: u16) -> Result<(), surrealdb::Error> {
        let result = self.client
            .query(format!("LET $now = time::now(); CREATE log:[{}, $now] SET specie=specie:{}, price={}, time=$now;", id, id, price))
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    #[allow(unused)]
    pub async fn clear_species(&self) {
        let _species: Result<Vec<Specie>, surrealdb::Error> = self.client.delete("specie").await;
    }

    #[allow(unused)]
    pub async fn clear_logs(&self) {
        let _species: Result<Vec<Specie>, surrealdb::Error> = self.client.delete("log").await;
    }
}
