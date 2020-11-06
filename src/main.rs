///////////////////////////////////////////////////////////////////////////////////////////////////
// Config setup for webhooks and other settings

mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Config {
        pub webhook_url: String,
        pub other_config: String,
    }

    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Req Functions

mod req_macros {
    use hyper::body;
    use hyper::{
        Client,
        Body,
        Method,
        Request,
        Uri
    };
    use serde_json::{
        Result,
        Value
    };
    use serde::{
        Serialize,
        Deserialize
    };
    use hyper_tls::HttpsConnector;
    use tokio;
    use pretty_env_logger;

    async fn find_variant(keywd: String) -> Result<&'static String, dyn std::error::Error> {

         // building client and request (has to use tls otherwise 304 status)
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let webhookurl = std::env::var("webhookurl").unwrap().as_str().parse()?;
        let mobile_endpoint: hyper::Uri = "https://www.supremenewyork.com/mobile_stock.json".parse::<Uri>().unwrap(); 

        let resp = client
            .get(mobile_endpoint.clone())
            .await?;

        //going from stream to str
        let bod_byte = body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(bod_byte.to_vec())
            .expect("resp not utf8");

        //so we can use this str as json
        let v: Value = serde_json::from_str(&body)?; 
     
        for (key, _value) in v["products_and_categories"].as_object().unwrap() {
            println!("{}", key);
            for value in v["products_and_categories"][key].as_array().unwrap(){
                if value["name"].as_str().unwrap().contains(&keywd) {
                    Ok(value["id"].as_str().unwrap());
                }
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//

