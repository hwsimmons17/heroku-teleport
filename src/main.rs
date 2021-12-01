#[macro_use]
extern crate rocket;

use rocket_sync_db_pools::{database, diesel};

pub mod routes;

// #[database("postgres_logs")]
// pub struct PgDB(diesel::PgConnection);

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/driver",
            routes![
                routes::driver::create_driver,
                routes::driver::start_driving,
                routes::driver::update_location,
                routes::driver::stop_driving
            ],
        )
        .mount(
            "/rider",
            routes![
                routes::rider::get_cost,
                routes::rider::paid,
                routes::rider::bump
            ],
        )
    // .attach(PgDB::fairing())
}
