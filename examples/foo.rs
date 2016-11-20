extern crate pinboard;

use pinboard::Client;

fn main() {
    let secret_key = include_str!("secret-key.ascii");
    let token = pinboard::AuthToken(secret_key.to_string());
    let client = Client::from(token);

    println!("{}", client.get_all());
    println!("{}", client.posts_update());
    println!("{}", client.tags_get());
}
