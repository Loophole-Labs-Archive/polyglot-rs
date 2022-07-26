extern crate polyglot;

use lazy_static::lazy_static;
use polyglot::Decoder;
use polyglot::Kind;
use serde::Deserialize;
use serde_json::Value;
use std::io::Cursor;
use std::sync::Mutex;
use std::sync::Once;

#[derive(Debug, Deserialize)]
struct RawTestData {
    name: String,
    kind: u8,
    #[serde(rename = "encodedValue")]
    encoded_value: String,
    #[serde(rename = "decodedValue")]
    decoded_value: Value,
}

struct TestData {
    name: String,
    kind: Kind,
    decoded_value: Value,
    encoded_value: Vec<u8>,
}

static TEST_DATA_URL: &str = "https://github.com/loopholelabs/polyglot-test-data/releases/download/unstable/polyglot-test-data.json";
static INIT: Once = Once::new();

lazy_static! {
    static ref TEST_DATA: Mutex<Vec<TestData>> = Mutex::new(vec![]);
}

fn init() {
    INIT.call_once(|| {
        reqwest::blocking::get(TEST_DATA_URL)
            .unwrap()
            .json::<Vec<RawTestData>>()
            .unwrap()
            .into_iter()
            .for_each(|td| {
                TEST_DATA.lock().unwrap().push(TestData {
                    name: td.name,
                    kind: Kind::from(td.kind),
                    decoded_value: td.decoded_value,
                    encoded_value: base64::decode(td.encoded_value).unwrap(),
                })
            });
    })
}

#[test]
fn test_encode() {
    init();

    let a: &mut Vec<TestData> = &mut TEST_DATA.lock().unwrap();

    for td in a {
        let mut decoder = Cursor::new(td.encoded_value.as_mut());

        match td.kind {
            Kind::None => {
                let val = decoder.decode_none();

                if td.decoded_value.is_null() {
                    assert_eq!(val, true)
                } else {
                    assert_eq!(val, false)
                }
            }

            Kind::Bool => {
                let val = decoder.decode_bool().unwrap();

                assert_eq!(val, td.decoded_value.as_bool().unwrap());
            }

            Kind::U8 => {
                let val = decoder.decode_u8().unwrap();

                assert_eq!(val as u64, td.decoded_value.as_u64().unwrap());
            }

            Kind::U16 => {
                let val = decoder.decode_u16().unwrap();

                assert_eq!(val as u64, td.decoded_value.as_u64().unwrap());
            }

            Kind::U32 => {
                let val = decoder.decode_u32().unwrap();

                assert_eq!(val as u64, td.decoded_value.as_u64().unwrap());
            }

            Kind::U64 => {
                let val = decoder.decode_u64().unwrap();

                assert_eq!(val as u64, td.decoded_value.as_u64().unwrap());
            }

            Kind::I32 => {
                let val = decoder.decode_i32().unwrap();

                assert_eq!(val as i64, td.decoded_value.as_i64().unwrap());
            }

            Kind::I64 => {
                let val = decoder.decode_i64().unwrap();

                assert_eq!(val as i64, td.decoded_value.as_i64().unwrap());
            }

            Kind::F32 => {
                let val = decoder.decode_f32().unwrap();

                assert!(
                    (val as f32 - td.decoded_value.as_f64().unwrap() as f32) < std::f32::EPSILON
                );
            }

            Kind::F64 => {
                let val = decoder.decode_f64().unwrap();

                assert!(
                    (val as f64 - td.decoded_value.as_f64().unwrap() as f64) < std::f64::EPSILON
                );
            }

            Kind::Array => {
                let len = decoder.decode_array(Kind::String).unwrap();

                let expected = td.decoded_value.as_array().unwrap();

                assert_eq!(expected.len(), len);

                for (i, _) in expected.into_iter().enumerate() {
                    assert_eq!(
                        expected[i].as_str().unwrap(),
                        decoder.decode_string().unwrap()
                    )
                }
            }

            Kind::Map => {
                let len = decoder.decode_map(Kind::String, Kind::U32).unwrap();

                let expected = td.decoded_value.as_object().unwrap();

                assert_eq!(expected.len(), len);

                for (expected_key, expected_value) in expected {
                    let actual_key = decoder.decode_string().unwrap();
                    let actual_value = decoder.decode_u32().unwrap();

                    assert_eq!(expected_key.as_str(), actual_key);
                    assert_eq!(expected_value.as_u64().unwrap(), actual_value as u64);
                }
            }

            Kind::Bytes => {
                let val = decoder.decode_bytes().unwrap();

                assert_eq!(
                    val,
                    base64::decode(td.decoded_value.as_str().unwrap()).unwrap()
                );
            }

            Kind::String => {
                let val = decoder.decode_string().unwrap();

                assert_eq!(val, td.decoded_value.as_str().unwrap());
            }

            Kind::Error => {
                let val = decoder.decode_error().unwrap();

                assert_eq!(val, td.decoded_value.as_str().unwrap());
            }

            _ => panic!("Unimplemented decoder for test {}", td.name),
        }
    }
}
