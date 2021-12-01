use rocket::serde::{json::Json, Deserialize, Serialize};

// use crate::PgDB;

#[derive(Serialize)]
pub struct StandardOutput<'r> {
    success: bool,
    message: &'r str,
}

#[derive(Deserialize)]
pub struct CreateDriverInput<'r> {
    pubkey: &'r str,
    password: &'r str,
}
#[derive(Deserialize)]
pub struct StartDrivingInput<'r> {
    pubkey: &'r str,
    time: i64,
    lat: i64,
    long: i64,
    signed_message: &'r str,
}
#[derive(Deserialize)]
pub struct StopDrivingInput<'r> {
    pubkey: &'r str,
    time: i64,
    signed_message: &'r str,
}

//this fails if pubkey is already in database
#[post("/registration-submission", data = "<req>")]
pub async fn registration_submission(req: Json<CreateDriverInput<'_>>) -> Json<StandardOutput<'_>> {
    if req.password != "password" {
        return Json(StandardOutput {
            success: false,
            message: "password not equal to \"password\"",
        });
    }

    Json(StandardOutput {
        success: true,
        message: "nothing happened",
    })
}

#[post("/create-driver", data = "<req>")]
pub fn create_driver(req: Json<CreateDriverInput<'_>>) -> Json<StandardOutput> {
    if req.password != "password" {
        return Json(StandardOutput {
            success: false,
            message: "password not equal to \"password\"",
        });
    }
    Json(StandardOutput {
        success: true,
        message: "nothing happened",
    })
}

#[post("/start-driving", data = "<req>")]
pub fn start_driving(req: Json<StartDrivingInput<'_>>) -> Json<StandardOutput> {
    Json(StandardOutput {
        success: true,
        message: "nothing happened",
    })
}

#[post("/update-location", data = "<req>")]
pub fn update_location(req: Json<StartDrivingInput<'_>>) -> Json<StandardOutput> {
    Json(StandardOutput {
        success: true,
        message: "nothing happened",
    })
}

#[post("/stop-driving", data = "<req>")]
pub fn stop_driving(req: Json<StopDrivingInput<'_>>) -> Json<StandardOutput> {
    Json(StandardOutput {
        success: true,
        message: "nothing happened",
    })
}
