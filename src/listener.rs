use std::net::Ipv4Addr;
use tokio::net::TcpListener;


pub async fn get_listener() -> (TcpListener, u16) {
    let port: u16 = std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
        .unwrap_or("3000".to_string())
        .parse()
        .expect("FUNCTIONS_CUSTOMHANDLER_PORT must be a number");

    let listener = TcpListener::bind(format!("{}:{}", Ipv4Addr::LOCALHOST, port))
        .await
        .unwrap();
	(listener, port)
}
