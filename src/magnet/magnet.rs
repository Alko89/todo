use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use self::schema::magnets;
use self::schema::magnets::dsl::{magnets as all_magnets};

mod schema {
    infer_schema!("env:DATABASE_URL");
}

#[table_name = "magnets"]
#[derive(Serialize, Queryable, Insertable, FromForm, Debug, Clone)]
pub struct Magnet {
    pub id: Option<i32>,
    pub magnet: String,
    pub seeders: Option<i32>,
    pub leechers: Option<i32>,
    pub name: String,
    pub website_source: String,
    pub url: String,
    pub size: String,
}


impl Magnet {
    pub fn all(conn: &SqliteConnection) -> Vec<Magnet> {
        all_magnets.order(magnets::id.desc()).load::<Magnet>(conn).unwrap()
    }

//     pub fn insert(&self, conn: &SqliteConnection) -> bool {
//         diesel::insert(self).into(posts::table).execute(conn).is_ok()
//     }
// 
//     pub fn toggle_with_id(id: i32, conn: &SqliteConnection) -> bool {
//         let post = all_posts.find(id).get_result::<Post>(conn);
//         if post.is_err() {
//             return false;
//         }
// 
//         let new_status = !post.unwrap().published.unwrap();
//         let updated_post = diesel::update(all_posts.find(id));
//         updated_post.set(post_published.eq(new_status)).execute(conn).is_ok()
//     }
// 
//     pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
//         diesel::delete(all_posts.find(id)).execute(conn).is_ok()
//     }
}
