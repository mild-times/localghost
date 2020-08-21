use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn beacon_can_send() {
    let beacon = localghost::net::Beacon::new("https://example.com".to_string());
    beacon.send(&mut b"hello world".to_owned());
}
