use rocket::fairing::AdHoc;
use crate::controller::notification;

pub fn route_stage() -> AdHoc {
    AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket.mount("/", routes![notification::subscribe])
    })
}
