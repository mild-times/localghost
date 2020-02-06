use wasm_bindgen_test::*;

#[wasm_bindgen_test]
async fn document_ready() {
    coast::log::Logger::new().start();
    coast::log::info!("almost ready");
    coast::ready().await;
    coast::log::info!("ready");
}
