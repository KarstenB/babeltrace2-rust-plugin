//! Rustified, object oriented abstraction layer for libbabeltrace2
//!
//! Provides an object oriented abstraction layer for the plain C API of [`babeltrace`].
//! The original API has the form `<comp>_<function>('self', ...)`. These functions are 
//! translated into rust structs with functions. `<comp>` is translated to a CamelCase
//! struct. And `<function>` are `pub fn` using the stored `'self'` pointer.
//! 
//! For example the function `bt_event_borrow_stream()` can be found in the struct `BtEvent`, 
//! together with the function `borrow_stream`.
//! 
//! [`babeltrace`]: https://babeltrace.org/docs/v2.0/libbabeltrace2/