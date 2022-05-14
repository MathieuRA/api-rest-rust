#[get("/ping")]
pub fn ping_rt() -> &'static str {
    "PONG!"
}