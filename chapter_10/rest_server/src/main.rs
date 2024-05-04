#![warn(clippy::all)]

use clap::Parser;
//// use dotenv::dotenv;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

use error_handlers::return_error;

mod profanity;
mod routes;
mod store;
mod types;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Which errors we want to log (info, warn or error)
    #[clap(short, long, default_value = "info")]
    log_level: String,
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    database_host: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    database_port: u16,
    /// Database name
    #[clap(long, default_value = "rustwebdev_db")]
    database_name: String,
    /// Database username
    #[clap(long, default_value = "rustwebdev")]
    database_username: String,
    /// Database password
    #[clap(long, default_value = "rustwebdev")]
    database_password: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), error_handlers::Error> {
    let args = Args::parse();

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rust_web_dev={},warp={}",
            args.log_level, args.log_level, args.log_level
        )
    });

    tracing::info!("Starting up...");
    tracing::info!("Reading .env file for environment variables...");
    dotenv::dotenv().ok();

    if let Err(_) = std::env::var("BAD_WORDS_API_KEY") {
        panic!("BadWords API key not set");
    }

    if let Err(_) = std::env::var("PASETO_KEY") {
        panic!("PASETO key not set");
    }

    if let Err(_) = std::env::var("DATABASE_URL") {
        panic!("DATABASE_URL key not set");
    }

    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))
        .map_err(|e| error_handlers::Error::ParseError(e))?;

    let default_db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        args.database_username,
        args.database_password,
        args.database_host,
        args.database_port,
        args.database_name
    );

    let db_url =
        dotenv::var("DATABASE_URL").ok().unwrap_or(default_db_url);
    println!("Database URL: {}", db_url);

    println!("Connecting to the database...");
    let store = store::Store::new(&db_url)
        .await
        .map_err(|e| error_handlers::Error::DatabaseQueryError(e))?;

    println!("Migrating the database...");
    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migrations");
    println!("Finished migrating the database!");

    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::PUT,
            Method::DELETE,
            Method::GET,
            Method::POST,
        ]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    let routes = get_questions
        .or(update_question)
        .or(add_question)
        .or(delete_question)
        .or(add_answer)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    tracing::info!(
        "Q&A service build ID {}",
        env!("RUST_WEB_DEV_VERSION")
    );

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    Ok(())
}
