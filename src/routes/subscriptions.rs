use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use tracing::{error, info};
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
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let query_span = tracing::info_span!(" Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    // First we attach the instrumentation, then we `.await` it
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
