extern crate hyper;

#[cfg(not(test))]
use hyper::client::Client as HClient;
#[cfg(not(test))]
use std::io::Read;
use std::fmt;

pub struct AuthToken (pub String);

impl fmt::Display for AuthToken {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct Client{
    token: AuthToken
}

impl Client {
    pub fn get_all(&self) -> String {
        let url = format!("https://api.pinboard.in/v1/posts/all?auth_token={}",self.token);
        get_url(&url)
    }
    pub fn posts_update(&self) -> String {
        let url = format!("https://api.pinboard.in/v1/posts/update?auth_token={}",self.token);
        get_url(&url)
    }
    pub fn tags_get(&self) -> String {
        let url = format!("https://api.pinboard.in/v1/tags/get?auth_token={}",self.token);
        get_url(&url)
    }
}
impl From<AuthToken> for Client {
    fn from(token: AuthToken) -> Client {
        Client{token:token}
     }
}

#[cfg(not(test))]
fn get_url(url: &str) -> String {
    let client = HClient::new();
    let mut result = String::new();

    let _res = client.get(url).send().unwrap().read_to_string(&mut result).unwrap();

    return result;
}

#[cfg(test)]
fn get_url(url: &str) -> String {
    match url {
        "https://api.pinboard.in/v1/posts/all?auth_token=test:test1234test" => "result1",
        "https://api.pinboard.in/v1/posts/update?auth_token=test:test1234test" => "result2",
        "https://api.pinboard.in/v1/tags/get?auth_token=test:test1234test" => "result3",
        _ => panic!("unexpected URL!")
    }.to_string()
}

#[test]
fn test_get_all() {
    let token = AuthToken("test:test1234test".to_string());
    let client = Client::from(token);

    println!("{}", client.get_all());
    println!("{}", client.posts_update());
    println!("{}", client.tags_get());
}
