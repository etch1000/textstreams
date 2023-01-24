#[macro_use] extern crate rocket;

use rocket::tokio::time::{Duration, interval};
use rocket::response::stream::TextStream;

// Produces an infinite series of Hellos, one per second
#[get("/infinite-hello")]
fn hello() -> TextStream![&'static str] {
    TextStream! {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            yield "hello";
            interval.tick().await;
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
