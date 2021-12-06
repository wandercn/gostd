#[test]
fn test_QueryEscape() {
    let source ="https://dev.detection.quantibio.com/reservation&response_type=code&scope=snsapi_userinfo#wechat_redirect";
    let escape = crate::net::url::QueryEscape(source);
    assert_eq!(escape, "https%3A%2F%2Fdev.detection.quantibio.com%2Freservation%26response_type%3Dcode%26scope%3Dsnsapi_userinfo%23wechat_redirect")
}

fn test_QueryUnescape() {
    let escape = "https%3A%2F%2Fdev.detection.quantibio.com%2Freservation%26response_type%3Dcode%26scope%3Dsnsapi_userinfo%23wechat_redirect";
    if let Ok(source) = crate::net::url::QueryUnescape(escape) {
        assert_eq!(source,"https://dev.detection.quantibio.com/reservation&response_type=code&scope=snsapi_userinfo#wechat_redirect".to_string());
    } else {
        println!("QueryUnescape Failed!")
    }
}
