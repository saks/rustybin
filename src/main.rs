#[rocket::main]
async fn main() {
    let _x = rustybin::server().launch().await.unwrap();
}
