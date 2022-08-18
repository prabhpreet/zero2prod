use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use tracing::{error, info};
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

/*
//Experimenting with 'static - see https://docs.rs/tracing/latest/tracing/trait.Subscriber.html
trait StaticTrait :'static {}

const usize_arr: [usize; 4] = [1usize, 2, 3, 4];

#[allow(unused)]
struct StaticOkStruct {
    integ: i32,
    non_negative: usize,
    static_str: &'static str,
    static_usize_slice: &'static [usize]
}

impl StaticTrait for StaticOkStruct {}

#[allow(unused)]
static static_ok: StaticOkStruct = StaticOkStruct {
    integ: -5,
    non_negative: 0,
    static_str: "hello",
    static_usize_slice: &usize_arr
};

struct StaticBreakingStruct<'a> {
    any_string: &'a str,
    any_usize_ref: &'a usize,
    any_usize_slice: &'a [usize]
}

// This breaks:
// impl <'a> StaticTrait for StaticBreakingStruct<'a> {}
*/

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if !is_valid_name(&form.name) {
        return HttpResponse::BadRequest().finish();
    }
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    // First we attach the instrumentation, then we `.await` it
    .await
    .map_err(|err| {
        tracing::error!("Fail to execute query: {:?}", err);
        err
    })?;
    Ok(())
}

pub fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();
    let is_too_long = s.graphemes(true).count() > 256;

    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}
