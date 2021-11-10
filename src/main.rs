use rand::Rng;
use rocket::response::status::BadRequest;

#[macro_use] extern crate rocket;

fn is_service_fails() -> bool {
    // Generates a random value within half-open [0, 10) range
    rand::thread_rng().gen_range(0..10) < 2
}

fn calculate_power(number: i64, power: u32) -> Result<String, BadRequest<String>> {
    if is_service_fails() {
        Err(BadRequest(Some("Something bad happened!".to_string())))
    } else {
        Ok(number.pow(power).to_string())
    }
}

#[get("/second-power?<base>")]
fn second_power(base: i64) -> Result<String, BadRequest<String>> {
    calculate_power(base, 2) 
}

#[get("/third-power?<base>")]
fn third_power(base: i64) -> Result<String, BadRequest<String>> {
    calculate_power(base, 3) 
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![second_power, third_power])
}
