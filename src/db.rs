use surrealdb::sql::{Array, Object, Value};
use surrealdb::kvs::Datastore;
use surrealdb::dbs::{Response, Session};
use std::{collections::BTreeMap, sync::Arc};
use crate::{utils::prelude::W, utils::error::Error, specie::Specie};

impl From<Object> for Specie {
    fn from(val: Object) -> Self {
        let id_str = val.get("id").unwrap().to_string();
        Specie {
            id: id_str.split(':')
                .collect::<Vec<&str>>()[1]
                .parse::<u32>()
                .unwrap(),
            name: val.get("name").unwrap().to_string(),
        }
    }
}

#[derive(Clone)]
pub struct DB {
    pub ds: Arc<Datastore>,
    pub ses: Session,
}

impl DB {
    pub async fn execute(
        &self,
        query: &str,
        vars: Option<BTreeMap<String, Value>>,
    ) -> Result<Vec<Response>, surrealdb::Error> {
        let res = self.ds.execute(query, &self.ses, vars, false).await?;
        Ok(res)
    }

    pub async fn add_specie(&self, lf_id: u32, name: &str) -> Result<Object, Error> {
        let sql = "CREATE type::thing('specie', $id) CONTENT $data";

        let data: BTreeMap<String, Value> = [
            ("name".into(), name.into()),
        ].into();

        let vars: BTreeMap<String, Value> = [
            ("id".into(), lf_id.into()),
            ("data".into(), data.into()),
        ].into();

        let res = self.ds.execute(sql, &self.ses, Some(vars), false).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn get_species(&self) -> Result<Vec<Object>, Error> {
        let sql = "SELECT * FROM specie ORDER BY name ASC;";

        let res = self.execute(sql, None).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }
}
