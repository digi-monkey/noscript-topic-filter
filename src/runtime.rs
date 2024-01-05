use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub pubkey: String,
    pub created_at: i64,
    pub kind: u16,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

impl Event {
    pub fn find_first_e_tag(&self) -> Option<&String> {
        // Iterate over each inner vector in tags
        for inner_vec in &self.tags {
            // Check if the inner vector is not empty
            if let Some(first_item) = inner_vec.first() {
                // Check if the first item's first character is "e"
                if first_item.starts_with("e") {
                    // If true, return the second item if it exists
                    return inner_vec.get(1);
                }
            }
        }
        // If no match is found, return None
        None
    }

    pub fn find_second_e_tag(&self) -> Option<&String> {
        // Counter to track the number of matching occurrences
        let mut count = 0;

        // Iterate over each inner vector in tags
        for inner_vec in &self.tags {
            // Check if the inner vector is not empty
            if let Some(first_item) = inner_vec.first() {
                // Check if the first item's first character is "e"
                if first_item.starts_with("e") {
                    // Increment the counter
                    count += 1;

                    // If it's the second occurrence, return the string after it
                    if count == 2 {
                        return inner_vec.get(1);
                    }
                }
            }
        }
        // If no second match is found, return None
        None
    }

    pub fn find_first_tag(&self, input: &str) -> Option<Vec<String>> {
        // Iterate over each inner vector in tags
        for inner_vec in &self.tags {
            // Check if the inner vector is not empty
            if let Some(first_item) = inner_vec.first() {
                // Check if the first item starts with the input string
                if first_item.starts_with(input) {
                    // Clone the inner vector and remove the input string
                    let result = inner_vec.iter().skip(1).cloned().collect();
                    return Some(result);
                }
            }
        }
        // If no match is found, return None
        None
    }
}

#[wasm_bindgen]
extern "C" {
    fn selfEvent() -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    fn getEventById(id: String) -> Promise;
}

pub fn get_self_event() -> Result<Event, JsValue> {
    let js_event: JsValue = selfEvent();
    let rust_event: Event = serde_wasm_bindgen::from_value(js_event)?;
    Ok(rust_event)
}

pub async fn get_event_by_id(id: String) -> Result<Event, JsValue> {
    let promise = getEventById(id);
    let js_event = JsFuture::from(promise).await?;
    let rust_event: Event = serde_wasm_bindgen::from_value(js_event)?;
    Ok(rust_event)
}
