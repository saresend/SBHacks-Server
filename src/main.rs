extern crate eventsource;
extern crate reqwest;
use eventsource::reqwest::Client;
use reqwest::Url;


fn main() {
    let client = Client::new(
        Url::parse("https://sbhacks-6e5fa.firebaseio.com/test.json").unwrap(),
    );
    for event in client {}
}
