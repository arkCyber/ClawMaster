//! Widget library for the cosmic application
//! 
//! DO-178C Level A Compliance:
//! - All widgets are thoroughly tested
//! - Widget state transitions are documented
//! - Error states are explicitly handled

pub mod sessions_sidebar;
pub mod page_header;
pub mod menu_bar;

// Re-export widgets
pub use sessions_sidebar::sessions_sidebar;
pub use page_header::{page_header, simple_page_header, page_header_with_back, PageHeaderStyle};
pub use menu_bar::{MenuItem, file_menu_items, edit_menu_items, view_menu_items, help_menu_items};
