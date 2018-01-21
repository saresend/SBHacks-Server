use std::fs::File;
use reqwest;
use std::io::{Write, Read};
use tesseract;
use serde_json::Value;
use serde_json;
use reqwest::Client;





#[get("/")]
fn index() -> &'static str {
    "Hello World!"
}


#[get("/textualize?<url>")]
fn get_text(url: &str) -> String {
    let url = "https://vision.googleapis.com/v1/images:annotate?key=AIzaSyBsRuTEEI8QwMyoxhUKxFu7dwICR0KTsko";
    let body = GoogleRequest::new(url);
    println!("{:?}", serde_json::to_string(&body));
    let mut response = Client::new().post(url).json(&body).send().unwrap();
    let mut res = String::new();
    response.read_to_string(&mut res).unwrap();
    res

}



#[derive(Serialize, Debug)]
pub struct GoogleRequest {
    pub requests: Vec<GoogIm>,
}


impl GoogleRequest {
    pub fn new(uri: &str) -> GoogleRequest {

        GoogleRequest {
            requests: vec![
                GoogIm {
                    image: GoogleImage { source: SecondLayer { imageUri: uri.to_owned() } },
                    features: vec![
                        GoogleFeature {
                            r_type: "TEXT_DETECTION".to_string(),
                            maxResults: 1,
                        },
                    ],
                },
            ],
        }

    }
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct GoogIm {
    pub image: GoogleImage,

    pub features: Vec<GoogleFeature>,
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct GoogleImage {
    pub source: SecondLayer,
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct SecondLayer {
    pub imageUri: String,
}



#[derive(Serialize)]
#[derive(Debug)]
pub struct GoogleFeature {
    #[serde(rename = "type")]
    r_type: String,
    maxResults: i32,
}
