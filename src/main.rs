#[macro_use]
extern crate rocket;

pub mod mongo;
pub mod routes;

// struct LogsDbConn(diesel::SqliteConnection);

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
}
