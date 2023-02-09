extern crate polyglot_rs;

mod tests;
use crate::tests::{
    Data, Decode, Encode, Request, RequestCorpus, Response, SearchResponse, SearchResponseResult,
    StockPrices, StockPricesSuperWrap, StockPricesWrapper, Test,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use base64::engine;

#[derive(Debug, Deserialize)]
struct GeneratorTestData {
    testall: String,
}

fn get_test_data() -> GeneratorTestData {
    return serde_json::from_slice::<GeneratorTestData>(
        &fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("resources")
                .join("test")
                .join("generator-test-data.json"),
        )
        .unwrap(),
    )
    .unwrap();
}

#[test]
fn test_decode() {
    let test_b64 = get_test_data();
    let mut poly_data = engine::decode(test_b64.testall).unwrap();
    let mut decoder = Cursor::new(poly_data.as_mut());
    tests::TestAll::decode(&mut decoder).unwrap().unwrap();
}

#[test]
fn test_encode() {
    let test_b64 = get_test_data();
    let poly_data = engine::decode(test_b64.testall).unwrap();

    let test = tests::TestAll {
        request: Request {
            message: "Hello".to_string(),
            corpus: RequestCorpus::Universal,
        },
        response: Response {
            message: "Hello".to_string(),
            test: Data {
                message: "Hello".to_string(),
                checker: Test::Potato,
            },
        },
        search_response: SearchResponse {
            results: Vec::from([SearchResponseResult {
                url: "https://www.google.com".to_string(),
                title: "Google".to_string(),
                snippets: ["Google is a search engine".to_string()].to_vec(),
            }]),
            results2: Vec::from([SearchResponseResult {
                url: "https://www.google.com".to_string(),
                title: "Google".to_string(),
                snippets: ["Google is a search engine".to_string()].to_vec(),
            }]),
            snippets: ["Google is a search engine".to_string()].to_vec(),
            snippets2: ["Google is a search engine".to_string()].to_vec(),
        },
        stock_prices_super_wrap: StockPricesSuperWrap {
            prices: HashMap::from([(
                "AAPL".to_string(),
                StockPricesWrapper {
                    s_prices: Vec::from([StockPrices {
                        prices: HashMap::from([("price".to_string(), 100.0)]),
                    }]),
                },
            )]),
        },
    };

    let mut encoder: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(512));
    test.encode(&mut encoder);
    assert_eq!(poly_data, encoder.into_inner());
}
