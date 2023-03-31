use auth_service_example::*;

fn main() {
    let credentials = create_credentials("adsf".to_string(), "adsf".to_string());
    authenticate(credentials);
}
