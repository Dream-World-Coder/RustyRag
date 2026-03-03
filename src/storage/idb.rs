use crate::utils::error::RustyRagError;
use js_sys::Promise;
/// src/storage/idb.rs
/// Manages the singleton IndexedDB connection using `web_sys`.
/// Handles raw byte storage, retrieval, and deletion of quantized vectors.
/// Ensures no redundant database instances are spawned on the client.
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Event, IdbDatabase, IdbOpenDbRequest};

const DB_NAME: &str = "RustyRagDB";
const STORE_NAME: &str = "vectors";
const DB_VERSION: u32 = 1;

pub struct StorageManager {
    db: Option<IdbDatabase>,
}

impl StorageManager {
    pub fn new() -> Self {
        Self { db: None }
    }

    /// Initializes the singleton IndexedDB and sets up the object store.
    pub async fn init(&mut self) -> Result<(), RustyRagError> {
        if self.db.is_some() {
            return Ok(());
        }

        let window = window().ok_or(RustyRagError::IdbError("No global window".into()))?;
        let idb_factory = window
            .indexed_db()
            .map_err(|_| RustyRagError::IdbError("IDB not supported".into()))?
            .ok_or(RustyRagError::IdbError("IDB not supported".into()))?;

        let request = idb_factory
            .open_with_u32(DB_NAME, DB_VERSION)
            .map_err(|_| RustyRagError::IdbError("Failed to open DB".into()))?;

        let onupgradeneeded = Closure::once(move |event: Event| {
            let req = event
                .target()
                .unwrap()
                .dyn_into::<IdbOpenDbRequest>()
                .unwrap();
            let db = req.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
            if !db.object_store_names().contains(STORE_NAME) {
                db.create_object_store(STORE_NAME)
                    .expect("Failed creating store");
            }
        });
        request.set_onupgradeneeded(Some(onupgradeneeded.as_ref().unchecked_ref()));

        let promise = Promise::new(&mut |resolve, reject| {
            let onsuccess = Closure::once(move |e: Event| {
                resolve.call1(&JsValue::NULL, &e.target().unwrap()).unwrap()
            });
            let onerror = Closure::once(move |e: Event| reject.call1(&JsValue::NULL, &e).unwrap());
            request.set_onsuccess(Some(onsuccess.as_ref().unchecked_ref()));
            request.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        });

        let event_target: web_sys::EventTarget = JsFuture::from(promise)
            .await
            .map_err(|_| RustyRagError::IdbError("IDB open rejected".into()))?
            .dyn_into()
            .unwrap();

        let request: IdbOpenDbRequest = event_target.dyn_into().unwrap();
        let db: IdbDatabase = request.result().unwrap().dyn_into().unwrap();

        self.db = Some(db);
        Ok(())
    }

    /// Stores a quantized vector array against an item-id.
    pub async fn store_kv(&self, id: &str, data: &[i8]) -> Result<(), RustyRagError> {
        let db = self
            .db
            .as_ref()
            .ok_or(RustyRagError::IdbError("DB not initialized".into()))?;
        let tx = db
            .transaction_with_str_and_mode(STORE_NAME, web_sys::IdbTransactionMode::Readwrite)
            .map_err(|_| RustyRagError::IdbError("Failed to create tx".into()))?;
        let store = tx.object_store(STORE_NAME).unwrap();

        // Avoid copy by mapping memory directly as Int8Array
        let js_data = unsafe { js_sys::Int8Array::view(data) };
        store
            .put_with_key(&js_data, &JsValue::from_str(id))
            .map_err(|_| RustyRagError::IdbError("Put request failed".into()))?;

        let promise = Promise::new(&mut |resolve, reject| {
            let oncomplete = Closure::once(move |_| resolve.call0(&JsValue::NULL).unwrap());
            let onerror = Closure::once(move |_| reject.call0(&JsValue::NULL).unwrap());
            tx.set_oncomplete(Some(oncomplete.as_ref().unchecked_ref()));
            tx.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        });

        JsFuture::from(promise)
            .await
            .map_err(|_| RustyRagError::IdbError("TX failed".into()))?;
        Ok(())
    }

    /// Deletes a specific item-id from the database.
    pub async fn delete_kv(&self, id: &str) -> Result<(), RustyRagError> {
        let db = self
            .db
            .as_ref()
            .ok_or(RustyRagError::IdbError("DB not initialized".into()))?;
        let tx = db
            .transaction_with_str_and_mode(STORE_NAME, web_sys::IdbTransactionMode::Readwrite)
            .map_err(|_| RustyRagError::IdbError("Failed to create tx".into()))?;
        let store = tx.object_store(STORE_NAME).unwrap();

        store
            .delete(&JsValue::from_str(id))
            .map_err(|_| RustyRagError::IdbError("Delete request failed".into()))?;

        let promise = Promise::new(&mut |resolve, reject| {
            let oncomplete = Closure::once(move |_| resolve.call0(&JsValue::NULL).unwrap());
            let onerror = Closure::once(move |_| reject.call0(&JsValue::NULL).unwrap());
            tx.set_oncomplete(Some(oncomplete.as_ref().unchecked_ref()));
            tx.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        });

        JsFuture::from(promise)
            .await
            .map_err(|_| RustyRagError::IdbError("Delete TX failed".into()))?;
        Ok(())
    }
}
