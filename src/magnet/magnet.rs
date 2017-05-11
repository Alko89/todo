use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use self::schema::magnets;
// use self::schema::magnets::dsl::{magnets as all_magnets};

mod schema {
    infer_schema!("env:DATABASE_URL");
}

#[table_name = "magnets"]
#[derive(Serialize, Queryable, Insertable, FromForm, Debug, Clone)]
pub struct Magnet {
    pub id: Option<i32>,
    pub magnet: String,
    pub seeders: i32,
    pub leechers: i32,
    pub name: String,
    pub website_source: String,
    pub url: String,
    pub size: String,
    pub inserted_at: String,
    pub updated_at: String,
}


impl Magnet {
    pub fn all(conn: &SqliteConnection) -> Vec<Magnet> {
        magnets::table.order(magnets::id.desc()).load::<Magnet>(conn).unwrap()
    }

    pub fn insert(&self, conn: &SqliteConnection) -> bool {
        diesel::insert(self).into(magnets::table).execute(conn).is_ok()
    }
}
