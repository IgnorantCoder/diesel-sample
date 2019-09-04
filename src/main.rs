#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
mod schema;

use uuid::*;
use chrono::*;
use diesel::prelude::*;
use crate::schema::*;

embed_migrations!();

#[derive(Insertable, Debug, Clone, AsChangeset)]
#[table_name = "users"]
pub struct InsertableUser<'a> {
    pub id: Uuid,
    pub screen_name: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone, AsChangeset)]
#[table_name = "posts"]
pub struct InsertablePost<'a> {
    pub id: Uuid,
    pub content: &'a str,
    pub id_users: Uuid,
}

#[derive(Identifiable, Queryable, Debug, Clone)]
#[primary_key(id)]
#[table_name = "users"]
pub struct QueryableUser {
    pub id: Uuid,
    pub screen_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, Associations, Debug, Clone)]
#[primary_key(id)]
#[belongs_to(QueryableUser, foreign_key="id_users")]
#[table_name = "posts"]
pub struct QueryablePost {
    pub id: Uuid,
    pub content: String,
    pub id_users: Uuid,
}

fn main() {
    let connection = diesel::pg::PgConnection::establish("postgres://postgres@db:5432/sample").unwrap();    
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();

    let nanoha = InsertableUser{ id: uuid(), screen_name: "nanoha", created_at: now() };
    let fate = InsertableUser{ id: uuid(), screen_name: "fate", created_at: now() };
    let vita = InsertableUser{ id: uuid(), screen_name: "vita", created_at: now() };
    let signam = InsertableUser{ id: uuid(), screen_name: "signam", created_at: now() };    // wrong name
    diesel::insert_into(users::table)
        .values(vec![
            &nanoha,
            &fate,
            &vita,
            &signam,
        ])
        .execute(&connection)
        .unwrap();

    let signum = InsertableUser{ id: signam.id, screen_name: "signum", created_at: signam.created_at };
    diesel::insert_into(users::table)
        .values(&signum)
        .on_conflict(users::id)
        .do_update()
        .set(&signum)
        .execute(&connection)
        .unwrap();

    diesel::insert_into(posts::table)
        .values(vec![
            &InsertablePost{ id: uuid(), content: "diary 9/4", id_users: nanoha.id },
            &InsertablePost{ id: uuid(), content: "diary 9/5", id_users: nanoha.id },
            &InsertablePost{ id: uuid(), content: "diary 9/10", id_users: nanoha.id },
            &InsertablePost{ id: uuid(), content: "photo00", id_users: fate.id },
            &InsertablePost{ id: uuid(), content: "photo01", id_users: fate.id },
            &InsertablePost{ id: uuid(), content: "battle memo", id_users: signum.id },
        ])
        .execute(&connection)
        .unwrap();
    
    let _nanoha_posts: Vec<QueryablePost> = posts::table
        .filter(posts::id_users.eq(nanoha.id))
        .load(&connection).unwrap();

    let user: QueryableUser = users::table.filter(users::screen_name.eq("fate")).first(&connection).unwrap();
    let fate_posts: Vec<QueryablePost> = QueryablePost::belonging_to(&user).load(&connection).unwrap();
    println!("{:?}", fate_posts);

}

fn uuid() -> Uuid {
    Uuid::new_v4()
}

fn now() -> NaiveDateTime {
    Local::now().naive_local()
}