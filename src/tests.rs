#[cfg(test)]
mod tests {
    use reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    };
    use std::time::Duration;
    use tokio::time::sleep;

    const URL: &str = "http://localhost:8080";

    fn get_client() -> Client {
        Client::builder()
            .connection_verbose(true)
            //.local_address(IpAddr::from_str("0.0.0.0").unwrap())
            .build()
            .unwrap()
    }
    fn get_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.append("Accept", HeaderValue::from_static("text/xml, application/xml, application/xhtml+xml, text/html;q=0.9, text/plain;q=0.8, text/css, image/png, image/jpeg, image/gif;q=0.8, application/x-shockwave-flash, video/mp4;q=0.9, flv-application/octet-stream;q=0.8, video/x-flv;q=0.7, audio/mp4, application/futuresplash, */*;q=0.5"));
        headers.append("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Android; U; en-US) AppleWebKit/533.19.4 (KHTML, like Gecko) AdobeAIR/33.0"));
        headers.append("Host", HeaderValue::from_static("gacha-plus.com"));
        headers.append("Refer", HeaderValue::from_static("app:/gacha_club.swf"));
        headers.append(
            "Content-Type",
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );
        headers
    }

    #[tokio::test]
    async fn test_random_character() {
        tokio::spawn(crate::run_server());

        sleep(Duration::from_secs(10)).await;

        let result = get_client()
            .post(URL.to_string() + "/GPscripts/randomcode.php")
            .headers(get_headers())
            .body("")
            .send()
            .await;
        let txt = result.unwrap().text().await.unwrap();
        if !txt.starts_with("systemResult=2") {
            panic!("Randomcode: {}", txt);
        }

        //-------------------------------------------------
        let result = get_client()
            .post(URL.to_string() + "/GPscripts/club_import.php")
            .headers(get_headers())
            .body("accountx=111LY6U")
            .send()
            .await;
        let txt = result.unwrap().text().await.unwrap();
        if !txt.starts_with("systemResult=2") {
            panic!("From OC: {}", txt);
        }

        //-------------------------------------------------
        let result = get_client()
            .post(URL.to_string() + "/GPscripts/club_import.php")
            .headers(get_headers())
            .body("accountx=%23FEMLOL")
            .send()
            .await;

        let txt = result.unwrap().text().await.unwrap();
        if !txt.starts_with("systemResult=2") {
            panic!("From FreeOC: {}", txt);
        }
    }
}
