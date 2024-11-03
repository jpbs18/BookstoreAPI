use rocket::{
    serde::{json::Json, Serialize, Deserialize},
    State,
    http::Status
};
use sea_orm::*;
use crate::auth::AuthenticatedUser;
use super::{Response, SuccessResponse, ErrorResponse};
use crate::entities::{prelude::*, book};
use sea_orm::prelude::DateTimeUtc;
use std::time::SystemTime;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ResBook {
  pub id: i32,
  pub title: String,
  pub year: Option<i32>,
  pub cover: Option<String>,
  pub author_id: i32
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RespBookList {
  pub total: usize,
  pub books: Vec<ResBook>
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqBook {
  author_id: i32,
  title: String,
  cover: Option<String>,
  year: Option<i32>
}

impl From<&book::Model> for ResBook {
    fn from(b: &book::Model) -> Self {
        Self {
            id: b.id,
            author_id: b.author_id,
            title: b.title.to_owned(),
            year: b.year.to_owned(),
            cover: b.cover.to_owned(),
        }
    }
}

#[get("/")]
pub async fn index(db: &State<DatabaseConnection>, _user: AuthenticatedUser) -> Response<Json<RespBookList>> {
    let db = db as &DatabaseConnection;

    let books = Book::find()
        .order_by_desc(book::Column::UpdatedAt)
        .all(db)
        .await?
        .iter()
        .map(ResBook::from)
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(RespBookList {
            total: books.len(),
            books
        })
    )))
}

#[get("/<id>")]
pub async fn show(db: &State<DatabaseConnection>, _user: AuthenticatedUser, id: i32) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let book = Book::find_by_id(id).one(db).await?;

    let book = match book {
        Some(b) => b,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "Cannot find a book with the specified ID.".to_string(),
            )))
        }
    };

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBook::from(&book))
    )))
}

#[post("/", data = "<req_book>")]
pub async fn create(db: &State<DatabaseConnection>, user: AuthenticatedUser, req_book: Json<ReqBook>) -> Response<Json<ResBook>> {
   let db = db as &DatabaseConnection;

    let book = book::ActiveModel {
        user_id: Set(user.id as i32),
        author_id: Set(req_book.author_id.to_owned()),
        title: Set(req_book.title.to_owned()),
        year: Set(req_book.year.to_owned()),
        cover: Set(req_book.cover.to_owned()),
        ..Default::default()
    };

    let book = book.insert(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResBook::from(&book))
    )))
}

#[put("/<id>", data = "<req_book>")]
pub async fn update(db: &State<DatabaseConnection>, _user: AuthenticatedUser, id: i32, req_book: Json<ReqBook>) -> Response<Json<ResBook>> {
    let db = db as &DatabaseConnection;

    let mut book: book::ActiveModel = match Book::find_by_id(id).one(db).await? {
        Some(b) => b.into(),
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No book with that specified ID.".to_string(),
            )))
        }
    };

    book.author_id = Set(req_book.author_id);
    book.title = Set(req_book.title.to_owned());
    book.year = Set(req_book.year.to_owned());
    book.cover = Set(req_book.cover.to_owned());
    book.updated_at = Set(Some(DateTimeUtc::from(SystemTime::now())));

    let book = book.update(db).await?;

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResBook::from(&book))
    )))
}

#[delete("/<id>")]
pub async fn delete(db: &State<DatabaseConnection>, _user: AuthenticatedUser, id: i32) -> Response<String> {
    let db = db as &DatabaseConnection;

    let book = match Book::find_by_id(id).one(db).await? {
        Some(b) => b,
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No book with that specified ID.".to_string(),
            )))
        }
    };

    book.delete(db).await?;

    Ok(SuccessResponse((Status::Ok, "Book deleted.".to_string())))
}