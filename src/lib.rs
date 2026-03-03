/// src/lib.rs
/// The primary WebAssembly export module for RustyRag.
/// Coordinates between IDB storage, quantization, and HNSW components.
/// Exposes lifecycle methods: init, add, delete, search, and crash reporting to JS.
pub mod index;
pub mod storage;
pub mod utils;

use index::hnsw::HnswIndex;
use storage::idb::StorageManager;
use storage::quantise::Quantizer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RustyRagClient {
    storage: StorageManager,
    quantizer: Quantizer,
    index: HnswIndex,
}

#[wasm_bindgen]
impl RustyRagClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            storage: StorageManager::new(),
            quantizer: Quantizer::new(),
            index: HnswIndex::new(1000), // Configurable cache size
        }
    }

    /// 1. Initialization
    #[wasm_bindgen]
    pub async fn init_db(&mut self) -> Result<(), JsValue> {
        // TODO: self.storage.init().await
        Ok(())
    }

    /// 2. ADD/STORE: Accept ID and vector, quantize, store in IDB.
    #[wasm_bindgen]
    pub async fn add_vector(&mut self, id: &str, vector: &[f32]) -> Result<(), JsValue> {
        // TODO: Quantize vector and store via self.storage
        Ok(())
    }

    /// 3. DELETE: Remove k-v pair from storage and index.
    #[wasm_bindgen]
    pub async fn delete_vector(&mut self, id: &str) -> Result<(), JsValue> {
        // TODO: Remove from IDB and HNSW graph
        Ok(())
    }

    /// 4. SEARCH: Perform blazing fast similarity search.
    #[wasm_bindgen]
    pub fn search(&mut self, query_vector: &[f32], limit: usize) -> Result<js_sys::Array, JsValue> {
        // TODO: Query HNSW and return JS Array of matched IDs.
        Ok(js_sys::Array::new())
    }

    /// 5. CRASH: Expose handler for JS/Server coordination upon collapse.
    #[wasm_bindgen]
    pub fn report_crash(&self, context: &str) -> Result<(), JsValue> {
        // TODO: Bundle internal state logs and send signal to server via fetch or callback.
        Ok(())
    }
}
