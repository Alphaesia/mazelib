mod location;
mod value;
mod manager;

pub use self::location::InlineCellLocation;
pub use self::value::InlineCellValue;
pub use self::value::InlineCellValueEdgeType;
pub use self::manager::BoxSpaceInlineCellManager;
pub use self::manager::BoxSpaceInlineCellManagerBuilder;