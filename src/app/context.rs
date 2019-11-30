//! Context containing vital application information
//!
//! ---
//! author: Andrew Evans
//! ---

use tokio::runtime::Runtime;


/// Context storing runtime and runtime metadata
pub struct Context{
    runtime: Runtime,
}


/// Context implementation
impl Context{

    /// Get an mutable `tokio::runtime::Runtime` reference
    pub fn get_runtime(&mut self) -> &mut Runtime{
        &mut self.runtime
    }

    /// Create a new context
    ///
    /// # Arguments
    /// * `runtime` - The runtime for the application
    pub fn new(runtime: Runtime) -> Context{
        Context{
            runtime: runtime,
        }
    }
}
