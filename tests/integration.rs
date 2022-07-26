extern crate polyglot;

use lazy_static::lazy_static;
use polyglot::Decoder;
use polyglot::Encoder;
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
        match td.kind {
            Kind::None => {
                let mut decoder = Cursor::new(td.encoded_value.as_mut());
                let val = decoder.decode_none();

                if td.decoded_value.is_null() {
                    assert_eq!(val, true)
                } else {
                    assert_eq!(val, false)
                }
            }

            // Kind::Array => todo!(),
            // Kind::Map => todo!(),
            // Kind::Any => todo!(),
            // Kind::Bytes => todo!(),
            // Kind::String => todo!(),
            // Kind::Error => todo!(),
            // Kind::Bool => todo!(),
            // Kind::U8 => todo!(),
            // Kind::U16 => todo!(),
            // Kind::U32 => todo!(),
            // Kind::U64 => todo!(),
            // Kind::I32 => todo!(),
            // Kind::I64 => todo!(),
            // Kind::F32 => todo!(),
            // Kind::F64 => todo!(),

            // _ => panic!("Unimplemented decoder for test {}", td.name),
            _ => {}
        }
    }
}
