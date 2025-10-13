pub mod aboutpage;
pub mod blogpage;
pub mod certifications;
pub mod footer;
pub mod header;
pub mod homepage;
pub mod notfoundpage;
pub mod onlineplaces;
pub mod page;
pub mod page_stats_display;
pub mod posts;

pub mod technologies;

// Re-export components for easier imports
pub use certifications::Certifications;
pub use onlineplaces::OnlinePlaces;
pub use page::Page;
pub use technologies::{Technologies, TechnologyType};
