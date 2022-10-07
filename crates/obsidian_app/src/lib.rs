mod app; 
pub use self::app::*;
pub mod logger; //if this disappears, we can't get logger functions in app.rs. Why is this necessary?

// lib.rs is represents the crate obsidian_app. It is at the top of the tree, so when
// things are brought into scope here, they are also in scope in other sibling modules.
// modules are not based on the file system, the only way rust knows a mod's relationship
// is by the mod x; syntax. 



