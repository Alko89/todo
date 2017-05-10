use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use self::schema::posts;
use self::schema::posts::dsl::{posts as all_posts, published as post_published};

mod schema {
    infer_schema!("env:DATABASE_URL");
}

#[table_name = "posts"]
#[derive(Serialize, Queryable, Insertable, FromForm, Debug, Clone)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: bool,
}


impl Post {
    pub fn all(conn: &SqliteConnection) -> Vec<Post> {
//         all_posts.filter(post_published.eq(true)).load::<Post>(conn).unwrap()
        all_posts.order(posts::id.desc()).load::<Post>(conn).unwrap()
    }
    
    pub fn post(id: i32, conn: &SqliteConnection) -> Vec<Post> {
        all_posts.find(id).load::<Post>(conn).unwrap()
    }

    pub fn insert(&self, conn: &SqliteConnection) -> bool {
        diesel::insert(self).into(posts::table).execute(conn).is_ok()
    }

    pub fn toggle_with_id(id: i32, conn: &SqliteConnection) -> bool {
        let post = all_posts.find(id).get_result::<Post>(conn);
        if post.is_err() {
            return false;
        }

        let new_status = !post.unwrap().published;
        let updated_post = diesel::update(all_posts.find(id));
        updated_post.set(post_published.eq(new_status)).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_posts.find(id)).execute(conn).is_ok()
    }
}
