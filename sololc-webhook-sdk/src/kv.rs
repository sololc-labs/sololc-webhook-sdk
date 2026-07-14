#[cfg(feature = "kv")]
pub mod kv {
    // 🌟 Imports the standard WASIp2 keyvalue bindings
    use wasi::keyvalue::store::{self, Bucket};

    /// Represents a safe, high-level wrapper over the WASIp2 `wasi:keyvalue` system.
    ///
    /// Manages an isolated namespace (or "bucket") within the host-provided data store. 
    /// This abstraction bypasses raw system handles and provides an ergonomic interface 
    /// similar to popular NoSQL databases, executing synchronously within the sandbox.
    pub struct KvStore {
        bucket: Bucket,
    }

    impl KvStore {
        /// Opens a named key-value storage bucket provided by the active host runtime.
        ///
        /// Returns a new instance of [`KvStore`] encapsulating the target bucket handle 
        /// upon success, or an [`Err(String)`][Err] if the bucket is inaccessible.
        ///
        /// # Errors
        ///
        /// This method returns an error if:
        /// - The requested `bucket_name` is empty or violates host security constraints.
        /// - The underlying host environment fails to initialize or allocate the requested storage.
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use sololc_webhook_sdk::kv::KvStore;
        ///
        /// # fn run() -> Result<(), String> {
        /// let store = KvStore::open("local_cache")?;
        /// # Ok(())
        /// # }
        /// ```
        pub fn open(bucket_name: &str) -> Result<Self, String> {
            let bucket = store::open(bucket_name)
                .map_err(|e| format!("Failed to open KV bucket '{}': {:?}", bucket_name, e))?;
            Ok(Self { bucket })
        }

        /// Fetches the raw byte payload associated with the specified key.
        ///
        /// Yields `Some(Vec<u8>)` if the key exists, `None` if it does not, or an 
        /// [`Err(String)`][Err] in the event of an operational failure.
        ///
        /// # Errors
        ///
        /// This method returns an error if:
        /// - The underlying physical database encounters an I/O or connection error.
        /// - The host runtime fails to serialize or marshal the retrieved values across the boundary.
        pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
            store::get(&self.bucket, key)
                .map_err(|e| format!("KV get failed for key '{}': {:?}", key, e))
        }

        /// Persists or updates a key-value entry within the storage bucket.
        ///
        /// Overwrites any existing value mapped to the target key.
        ///
        /// # Errors
        ///
        /// This method returns an error if:
        /// - The storage bucket is configured as read-only.
        /// - The host runtime runs out of disk space or storage quota limits are exceeded.
        /// - The physical database fails to persist the byte payload.
        pub fn set(&self, key: &str, value: &[u8]) -> Result<(), String> {
            store::set(&self.bucket, key, value)
                .map_err(|e| format!("KV set failed for key '{}': {:?}", key, e))
        }

        /// Removes a key-value entry permanently from the storage bucket.
        ///
        /// # Errors
        ///
        /// This method returns an error if:
        /// - The storage bucket is configured as read-only.
        /// - The underlying physical database fails to remove the record.
        pub fn delete(&self, key: &str) -> Result<(), String> {
            store::delete(&self.bucket, key)
                .map_err(|e| format!("KV delete failed for key '{}': {:?}", key, e))
        }
    }
}