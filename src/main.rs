use email_newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to find a port.");

    run(listener)?.await
}
