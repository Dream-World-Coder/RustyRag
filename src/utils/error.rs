/// src/utils/error.rs
/// Contains the centralized error handling for RustyRag.
/// Defines `RustyRagError` covering IDB failures, WASM boundary issues, and index errors.
/// Implements `Into<JsValue>` to easily pass errors back to JavaScript environments.
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum RustyRagError {
    IdbError(String),
    WasmError(String),
    QuantizationError(String),
    IndexError(String),
}

impl From<RustyRagError> for JsValue {
    fn from(err: RustyRagError) -> Self {
        match err {
            RustyRagError::IdbError(msg) => JsValue::from_str(&format!("IDB Error: {}", msg)),
            RustyRagError::WasmError(msg) => JsValue::from_str(&format!("WASM Error: {}", msg)),
            RustyRagError::QuantizationError(msg) => {
                JsValue::from_str(&format!("Quantization Error: {}", msg))
            }
            RustyRagError::IndexError(msg) => JsValue::from_str(&format!("Index Error: {}", msg)),
        }
    }
}
