use super::Msg;

// -- modules
mod add_server_form;
mod browser;
mod command_line;
mod editor;
mod error_result;
mod execute_result_table;
mod global_keys;
mod label;
mod settings;
mod snippets_table;
mod status;
mod tabs;

// -- export
pub(crate) use add_server_form::input_text::InputText;
pub(crate) use add_server_form::AddServerForm;
pub(crate) use add_server_form::FormSubmitListener;
pub(crate) use browser::BrowserTree;
pub(crate) use browser::SentTree;
pub use editor::Editor;
pub(crate) use error_result::ErrorResult;
pub(crate) use command_line::CommandLine;
pub(crate) use execute_result_table::ExecuteResultTable;
pub(crate) use settings::SettingsForm;
pub(crate) use snippets_table::SnippetsTable;
pub(crate) use status::DbResponseStatusListener;
pub(crate) use status::PressedKey;
pub(crate) use status::StatusSpan;
pub(crate) use status::StatusSpinner;

pub use global_keys::GlobalListener;
pub use label::Label;
pub(crate) use tabs::EditorTabs;
pub(crate) use tabs::ACTIVE_TAB_INDEX;
