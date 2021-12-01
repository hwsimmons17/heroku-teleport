use anchor_lang::prelude::*;
use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::helpers::verify::verify;

#[derive(Deserialize)]
pub struct req<'r> {
    complete: &'r str,
}

#[derive(Serialize)]
pub struct Something<'r> {
    response: &'r str,
}

#[derive(Serialize)]
pub struct StandardOutput<'r> {
    success: bool,
    message: &'r str,
}
#[derive(Serialize)]
pub struct GetCostOutput<'r> {
    success: bool,
    message: &'r str,
    cost: u64,
}

#[derive(Deserialize)]
pub struct GetCost {
    pubkey: Pubkey,
    current_lat: i64,
    current_long: i64,
    destination_lat: i64,
    destination_long: i64,
    time: i64,
}
#[derive(Deserialize)]
pub struct GetCostInput<'r> {
    message: &'r str,
    signature: &'r str,
}
#[derive(Deserialize)]
pub struct PaidInput<'r> {
    pubkey: &'r str,
    payment_id: &'r str,
    time: i64,
    signed_message: &'r str,
}

#[post("/get-cost", data = "<req>")]
pub fn get_cost(req: Json<GetCostInput<'_>>) -> Json<GetCostOutput> {
    let verified = verify(req.message, req.signature);

    if !verified.success {
        return Json(GetCostOutput {
            success: false,
            message: "verification failed",
            cost: 0,
        });
    }

    Json(GetCostOutput {
        success: true,
        message: "you are verified",
        cost: 100000000,
    })
}

#[post("/paid", data = "<req>")]
pub fn paid(req: Json<PaidInput<'_>>) -> Json<StandardOutput> {
    Json(StandardOutput {
        success: true,
        message: "nothing happened",
    })
}

#[post("/bump", data = "<req>")]
pub fn bump(req: Json<GetCostInput<'_>>) -> Json<GetCostOutput> {
    Json(GetCostOutput {
        success: true,
        message: "nothing happened, this is cost in lamports",
        cost: 100000000,
    })
}
