//! Config for setting the prefetch limit
//!
//! ---
//! author: Andrew Evans
//!


/// Config for setting the prefetch limit
pub struct SetPrefetchLimit{
    pub limit: u16,
    pub global: bool,
}
