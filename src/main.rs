use rocket::fs::FileServer;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = rocket::build()
        .mount("/", FileServer::from("static"))
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port: 10000,
            ..rocket::Config::default()
        });

    match rocket.launch().await {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    };
}
