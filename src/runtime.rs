use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub created_at: i64,
    pub kind: u16,
    tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

#[wasm_bindgen]
impl Event {
    #[wasm_bindgen(getter_with_clone)]
    pub fn get_tags(&self) -> Vec<Vec<String>> {
        return self.tags;
    }
}

#[wasm_bindgen]
extern "C" {
    fn selfEvent() -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    fn getEventById(id: String) -> JsValue;
}

#[wasm_bindgen]
pub fn get_self_event() -> Result<Event, JsValue> {
    let js_event: JsValue = selfEvent();
    let rust_event: Event = serde_wasm_bindgen::from_value(js_event)?;
    Ok(rust_event)
}

#[wasm_bindgen]
pub fn get_event_by_id(id: String) -> Result<Event, JsValue> {
    let js_event: JsValue = getEventById(id);
    let rust_event: Event = serde_wasm_bindgen::from_value(js_event)?;
    Ok(rust_event)
}
