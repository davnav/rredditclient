
use reqwest:: {Client, ClientBuilder, Response, Result, header};
use reqwest::header::{HeaderMap};
use reqwest::header:: { USER_AGENT,AUTHORIZATION } ;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct AuthData {
    pub access_token: String,
}

#[derive(Debug)]
pub struct AuthPassword{
    client_id:String,
    client_secret:String,
}
impl AuthPassword{
    fn new () -> Self{
        let c_id = env::var("client_id").unwrap();
        let c_secret = env::var("client_secret").unwrap();
        AuthPassword {
                client_id: c_id,
                client_secret: c_secret,
        }

    }
}


#[derive(Debug)]
pub struct RedditClient {
    pub client : Client,
    pub auth : AuthPassword,

}
impl RedditClient {
    pub fn new() -> Self {
        let client      = reqwest::Client::new();
        let auth   = AuthPassword::new();
        RedditClient{
            client:client,
            auth:auth,
        }
    }

    pub async fn connect(&mut self) {
        // let res = c.get("https://www.reddit.com/").send().await?;
        let url           = "https://www.reddit.com/api/v1/access_token";
        // println!("{:?}",&self.auth.client_id);
        let body   =  format!( "grant_type=password&username={}&password={}","naveendavisv".to_string(),"Myrpass21!".to_string() );
        let mut Myheader = HeaderMap::new();
        let username = env::var("username").unwrap();
        let password = env::var("password").unwrap();
        let form = [
            ("grant_type","password"),
            ("username",username.as_str()),
            ("password", password.as_str())
        ];
        Myheader.insert(USER_AGENT,"MyApi".parse().unwrap());
        // headers.insert("redirect_uri","https://twitter.com/naveendavis11?lang=en".parse().unwrap());

        let token_req = self.client.post(url).headers(Myheader.clone()).basic_auth(self.auth.client_id.clone(),Some(self.auth.client_secret.clone())).form(&form);

        let mut token_resp = token_req.send().await.unwrap();

        if token_resp.status() == 200 {
            let auth_data = token_resp.json::<AuthData>().await.unwrap();
            println!("{:?}",&auth_data);
            let tok = format!( "bearer {}",auth_data.access_token.to_owned());
            Myheader.insert(AUTHORIZATION,tok.to_owned().parse().unwrap());
            
        }

        // let s = serde_json::from_str(token_resp).unwrap();
        // for i in token_resp.headers().iter(){
        //     println!("{:?}",i);
        // }
        // let mut buf = String::new();
        
    
        // let buf = token_resp.access_token.as_str();


    //    println!("{:?}",&token_resp) ;

       println!("{:?}",&Myheader);
       let first_req = self.client.get("https://oauth.reddit.com/api/v1/me").headers(Myheader.clone()).send().await.unwrap();

       println!("{:?}",first_req);
        
        
    }
    
}







#[cfg(test)]
mod tests {
    use super:: *;
    #[tokio::test]
    async fn redditconnect() {

        let mut client = RedditClient::new();
        client.connect().await;
        
    }
}
