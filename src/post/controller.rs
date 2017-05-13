extern crate rocket;
extern crate rocket_contrib;

use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

use db;
use post::Post;


#[derive(Debug, Serialize)]
struct Context<'a, 'b> 
{
    msg: Option<(&'a str, &'b str)>,
    titles: Vec<Post>,
    posts: Vec<Post>
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn one(id: i32, conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context{
            msg: msg,
            titles: Post::all(conn),
            posts: Post::post(id, conn)
        }
    }
}



#[get("/")]
fn post(msg: Option<FlashMessage>, conn: db::Conn) -> Template {
    Template::render("index",
        &match msg {
        Some(ref msg) => Context::one(1, &conn, Some((msg.name(), msg.msg()))),
        None => Context::one(1, &conn, None),
        })
}

#[get("/post/<id>")]
fn view_post(id: i32  ,msg: Option<FlashMessage>, conn: db::Conn) -> Template {
    Template::render("index",
        &match msg {
        Some(ref msg) => Context::one(id, &conn, Some((msg.name(), msg.msg()))),
        None => Context::one(id, &conn, None),
        })
}

#[get("/add")]
fn add_post(msg: Option<FlashMessage>, conn: db::Conn) -> Template {
     Template::render("add_post", &match msg {
        Some(ref msg) => Context::one(1, &conn, Some((msg.name(), msg.msg()))),
        None => Context::one(1, &conn, None),
        })
}

#[post("/new", data = "<post_form>")]
fn new_post(post_form: Form<Post>, conn: db::Conn) -> Flash<Redirect> {
    let post = post_form.into_inner();
    if post.title.is_empty()
    {
        Flash::error(Redirect::to("/add"), "Title cannot be empty.")
    }
    else if post.body.is_empty()
    {
        Flash::error(Redirect::to("/add"), "Body cannot be empty.")
    } 
    else if post.insert(&conn)
    {
        Flash::success(Redirect::to("/post/1"), "Post successfully added.")
    } 
    else
    {
        Flash::error(Redirect::to("/add"), "Whoops! The server failed.")
    }
}

#[get("/edit/<id>")]
fn edit_post(id: i32, msg: Option<FlashMessage>, conn: db::Conn) -> Template {
     Template::render("edit_post", &match msg {
        Some(ref msg) => Context::one(id, &conn, Some((msg.name(), msg.msg()))),
        None => Context::one(id, &conn, None),
        })
}

#[post("/update", data = "<post_form>")]
fn update_post(post_form: Form<Post>, conn: db::Conn) -> Flash<Redirect> {
    let post = post_form.into_inner();
    if post.title.is_empty()
    {
        Flash::error(Redirect::to("/new"), "Title cannot be empty.")
    }
    else if post.body.is_empty()
    {
        Flash::error(Redirect::to("/new"), "Body cannot be empty.")
    } 
    else if Post::update(post.id.unwrap(), post.body, &conn)
    {
        Flash::success(Redirect::to(&format!("/post/{}", post.id.unwrap())), "Post successfully updated.")
    } 
    else
    {
        Flash::error(Redirect::to("/new"), "Whoops! The server failed.")
    }
}
