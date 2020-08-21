use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn document_ready() {
    localghost::log::Logger::new().start();
    localghost::log::info!("almost ready");
    localghost::ready().await;
    localghost::log::info!("ready");
}
