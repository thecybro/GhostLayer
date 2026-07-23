use web_sys::window;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub async fn copy_to_clipboard(text: String) -> Result<(), JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No global window found"))?;
    let clipboard = window.navigator().clipboard();

    JsFuture::from(clipboard.write_text(&text)).await?;

    Ok(())
}
