//! ## Components
//!
//! demo example components

use super::Msg;

use tuirealm::props::{Alignment, Borders, Color, Style};

use tuirealm::tui::widgets::Block;

// -- modules
mod add_server_form;
mod browser;
mod editor;
mod execute_result_table;
mod error_result;
mod exit;
mod label;
mod tabs;

// -- export
pub(crate) use add_server_form::input_text::InputText;
pub(crate) use add_server_form::AddServerForm;
pub(crate) use add_server_form::FormSubmitListener;
pub use browser::BrowserTree;
pub(crate) use browser::SentTree;
pub use editor::Editor;
pub(crate) use execute_result_table::ExecuteResultTable;
pub(crate) use error_result::ErrorResult;
pub(crate) use execute_result_table::QueryResult;
pub use exit::GlobalListener;
pub use label::Label;
pub(crate) use tabs::EditorTabs;
pub(crate) use tabs::ACTIVE_TAB_INDEX;
