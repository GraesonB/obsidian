//So this is in the src directory for the entire project, 
// "pub mod app" says here's a new module, store it for future use
// "pub use obsidian_app::*" says all of the public items in obsidian_app can be used by
// calling obsidian(the current directory)::app::* 

pub mod app {
    pub use obsidian_app::*; // This is allowed through our workspace dependencies.
}

pub mod render {
    pub use obsidian_render::*;
}