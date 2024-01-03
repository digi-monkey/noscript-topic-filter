extern crate bayespam;

//mod classifier;
use bayespam::classifier::{self, Classifier};
use js_sys::Reflect;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_valid_event(event: JsValue) -> bool {
    let classifier: Classifier = Classifier::new();
    if let Some(obj) = event.dyn_ref::<js_sys::Object>() {
        if let Ok(content) = Reflect::get(obj, &JsValue::from_str("content")) {
            if let Some(content) = content.as_string() {
                let is_spam = classifier.identify(content.as_str());
                if is_spam {
                    return true;
                }
            }
        }
    }
    false
}
