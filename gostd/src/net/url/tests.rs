use crate::net::url;
use std::collections::HashMap;
use std::io::Result;

#[test]
fn test_QueryEscape() {
    let source ="https://dev.detection.quantibio.com/reservation&response_type=code&scope=snsapi_userinfo#wechat_redirect";
    let escape = url::QueryEscape(source);
    assert_eq!(escape, "https%3A%2F%2Fdev.detection.quantibio.com%2Freservation%26response_type%3Dcode%26scope%3Dsnsapi_userinfo%23wechat_redirect")
}
#[test]
fn test_QueryUnescape() {
    let escape = "https%3A%2F%2Fdev.detection.quantibio.com%2Freservation%26response_type%3Dcode%26scope%3Dsnsapi_userinfo%23wechat_redirect";
    if let Ok(source) = url::QueryUnescape(escape) {
        assert_eq!(source,"https://dev.detection.quantibio.com/reservation&response_type=code&scope=snsapi_userinfo#wechat_redirect".to_string());
    } else {
        println!("QueryUnescape Failed!")
    }
}
#[test]
fn Test_ParseQuery() {
    let query = "a=1";
    if let Ok(out) = url::ParseQuery(query) {
        let values = url::Values::new(HashMap::from([("a".to_string(), vec!["1".to_string()])]));
        assert_eq!(out == values, true);
    } else {
        println!("ParseQuery failed!")
    }

    let query = "a=1&b=2";
    if let Ok(out) = url::ParseQuery(query) {
        let values = url::Values::new(HashMap::from([
            ("a".to_string(), vec!["1".to_string()]),
            ("b".to_string(), vec!["2".to_string()]),
        ]));
        assert_eq!(out == values, true);
    } else {
        println!("ParseQuery failed!")
    }

    let query = "a=1&a=2&a=banana";
    if let Ok(out) = url::ParseQuery(query) {
        let values = url::Values::new(HashMap::from([(
            "a".to_string(),
            vec!["1".to_string(), "2".to_string(), "banana".to_string()],
        )]));
        assert_eq!(out == values, true);
    } else {
        println!("ParseQuery failed!")
    }

    let query = "ascii=%3Ckey%3A+0x90%3E";
    if let Ok(out) = url::ParseQuery(query) {
        let values = url::Values::new(HashMap::from([(
            "ascii".to_string(),
            vec!["<key: 0x90>".to_string()],
        )]));
        assert_eq!(out == values, true);
    } else {
        println!("ParseQuery failed!")
    }

    let query = "a%3Bb=1";
    if let Ok(out) = url::ParseQuery(query) {
        let values = url::Values::new(HashMap::from([("a;b".to_string(), vec!["1".to_string()])]));
        assert_eq!(out == values, true);
    } else {
        println!("ParseQuery failed!")
    }

    let query = "a=%3B";
    if let Ok(out) = url::ParseQuery(query) {
        let values = url::Values::new(HashMap::from([("a".to_string(), vec![";".to_string()])]));
        assert_eq!(out == values, true);
    } else {
        println!("ParseQuery failed!")
    }
}
