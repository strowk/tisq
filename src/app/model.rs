//! ## Model
//!
//! app model

use crate::app::event_dispatcher::EventDispatcherPort;
use crate::components::{
    AddServerForm, BrowserTree, Editor, EditorTabs, ErrorResult, ExecuteResultTable,
    FormSubmitListener, GlobalListener, InputText, SentTree, ACTIVE_TAB_INDEX,
};

use super::connection::{self, DbRequest, DbResponse};
use super::storage::{NewServer, Storage};
use super::{storage, Id, Msg, TisqEvent};
use ordered_hash_map::OrderedHashMap;
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, vec};
use tui_realm_treeview::{Node, Tree};
use tuirealm::event::{Key, KeyEvent, KeyEventKind, KeyModifiers};
use tuirealm::props::{PropPayload, PropValue, TextSpan};
use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::{
    Application, AttrValue, Attribute, EventListenerCfg, Sub, SubClause, SubEventClause, Update,
};
use tuirealm::{Event, State, StateValue};
use uuid::Uuid;

pub(crate) type TisqApplication = Application<Id, Msg, TisqEvent>;

enum ExecuteResultState {
    FetchedTable,
    Error,
}

pub struct Model {
    /// Application
    pub(crate) app: TisqApplication,
    /// Indicates that the application must quit
    pub quit: bool,
    /// Tells whether to redraw interface
    pub redraw: bool,
    /// Used to draw to terminal
    pub terminal: TerminalBridge,

    adding_server: bool,
    add_server_form_mounted: bool,

    storage: Storage,

    event_dispatcher_port: EventDispatcherPort<TisqEvent>,
    add_server_form: AddServerForm,

    connection_manager_tx: Sender<DbRequest>,

    query_editors: OrderedHashMap<EditorId, EditorMetadata>,
    shown_editor: Option<EditorId>,
    // connection_manager_rx: Receiver<DbResponse>,
    // connections: HashMap<Uuid, Connection>,
    execute_result_state: ExecuteResultState,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct EditorId {
    server_id: Uuid,
    database: String,
}

struct EditorMetadata {
    name: String,
}

impl Default for Model {
    fn default() -> Self {
        let storage = storage::Storage::open(std::path::Path::new("./storage")).unwrap();

        let event_dispatcher = EventDispatcherPort::new();

        let (tx, rx): (Sender<DbRequest>, Receiver<DbRequest>) = mpsc::channel();
        let (back_tx, back_rx): (Sender<DbResponse>, Receiver<DbResponse>) = mpsc::channel();

        thread::spawn(|| {
            let connections_manager = connection::ConnectionsManager::new(back_tx);
            connections_manager.requests_loop(rx);
        });

        let back_rx = Arc::new(Mutex::new(back_rx));
        let mut back_dispatcher = event_dispatcher.clone();

        thread::spawn(move || loop {
            let response = back_rx.lock().unwrap().recv();
            match response {
                Ok(response) => {
                    back_dispatcher.dispatch(Event::User(TisqEvent::DbResponse(response)));
                }
                Err(e) => {
                    tracing::error!("Error receiving response from connection manager: {:?}", e);
                }
            }
        });

        Self {
            app: Self::init_app(&storage, Box::new(event_dispatcher.clone())),
            quit: false,
            redraw: true,
            terminal: TerminalBridge::new().expect("Cannot initialize terminal"),
            storage,

            adding_server: false,
            add_server_form_mounted: false,

            event_dispatcher_port: event_dispatcher,

            add_server_form: AddServerForm::new(),

            connection_manager_tx: tx,

            query_editors: OrderedHashMap::new(),
            shown_editor: None,
            // connection_manager_rx: back_rx,
            // connections: HashMap::new(),
            execute_result_state: ExecuteResultState::FetchedTable,
        }
    }
}

impl Model {
    pub fn view(&mut self) {
        let active_editor_id = self.get_or_set_shown_editor_id();

        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let sides = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Length(50),
                        Constraint::Min(0), // fills remaining space
                    ])
                    .split(f.size());

                let left = sides[0];
                let right = sides[1];
                // let chunks = Layout::default()
                //     .direction(Direction::Vertical)
                //     .margin(1)
                //     .constraints(
                //         [
                //             Constraint::Length(3), // Clock
                //             Constraint::Length(3), // Letter Counter
                //             Constraint::Length(3), // Digit Counter
                //             Constraint::Length(1), // Label
                //         ]
                //         .as_ref(),
                //     )
                //     .split(left);
                // self.app.view(&Id::Clock, f, chunks[0]);
                // self.app.view(&Id::LetterCounter, f, chunks[1]);
                // self.app.view(&Id::DigitCounter, f, chunks[2]);
                // self.app.view(&Id::Label, f, chunks[3]);
                self.app.view(&Id::Tree, f, left);

                if self.adding_server {
                    // self.app.view(&Id::AddServerForm, f, right);
                    self.add_server_form.view(right, &mut self.app, f);
                } else {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(1)
                        .constraints(
                            [
                                Constraint::Length(1),      // Tabs
                                Constraint::Percentage(60), // Editor
                                Constraint::Min(0),         // Result
                            ]
                            .as_ref(),
                        )
                        .split(right);

                    self.app.view(&Id::EditorTabs, f, chunks[0]);
                    if let Some(id) = active_editor_id {
                        self.app.view(&id, f, chunks[1]);
                    }
                    match self.execute_result_state {
                        ExecuteResultState::Error => {
                            self.app.view(&Id::ExecuteErrorResult, f, chunks[2]);
                        }
                        ExecuteResultState::FetchedTable => {
                            self.app.view(&Id::QueryResultTable, f, chunks[2]);
                        }
                    }
                }
            })
            .is_ok());
    }

    fn browser_tree(storage: &storage::Storage) -> eyre::Result<Node> {
        let mut node: Node = Node::new("root".to_string(), "servers".to_string());
        storage.read_servers()?.into_iter().for_each(|server| {
            let mut server_node: Node = Node::new(format!("server:{}", server.id), server.name);

            // dummy is created to make the server node expandable
            // it is a workaround for the limitation of the treeview component
            let dummy: Node = Node::new(
                format!("dummy:{}", server.id),
                "loading databases...".to_string(),
            );
            server_node.add_child(dummy);

            node.add_child(server_node);
        });
        Ok(node)
    }

    fn update_browser(&mut self) {
        let root = Self::browser_tree(&self.storage).unwrap();
        self.event_dispatcher_port
            .send_tree(SentTree(Tree::new(root)));
    }

    fn dir_tree(p: &Path, depth: usize) -> Node {
        let name: String = match p.file_name() {
            None => "/".to_string(),
            Some(n) => n.to_string_lossy().into_owned().to_string(),
        };
        let mut node: Node = Node::new(p.to_string_lossy().into_owned(), name);
        // println!("preparing node {:?}", node);
        // node.exp
        if depth > 0 && p.is_dir() {
            if let Ok(e) = std::fs::read_dir(p) {
                e.flatten()
                    .for_each(|x| node.add_child(Self::dir_tree(x.path().as_path(), depth - 1)));
            }
        }
        node
    }

    fn mount_editor(&mut self, id: EditorId) {
        assert!(self
            .app
            .mount(Id::Editor(id.clone()), Box::new(Editor::new(id)), vec![])
            .is_ok());
    }

    fn mount_editor_envelope(app: &mut TisqApplication) {
        // Mount tabs
        assert!(app
            .mount(Id::EditorTabs, Box::new(EditorTabs::new()), vec![])
            .is_ok());

        // Mount editor
        // assert!(app
        //     .mount(Id::Editor, Box::new(Editor::default()), vec![])
        //     .is_ok());

        // Mount query result table
        assert!(app
            .mount(
                Id::QueryResultTable,
                Box::new(ExecuteResultTable::default()),
                vec![
                    Sub::new(
                        SubEventClause::User(TisqEvent::DbResponse(
                            // the content does not matter due to the PartialEq implementation
                            DbResponse::Executed(Uuid::new_v4(), vec![], vec![]) // <- this is dummy
                        )),
                        SubClause::Always
                    ),
                    // Sub::new(SubEventClause::Tick, SubClause::Always)
                    Sub::new(SubEventClause::WindowResize, SubClause::Always)
                ] // vec![Sub::new(
                  //     SubEventClause::User(TisqEvent::QueryResultFetched(QueryResult {
                  //         data: vec![] // the content does not matter due to the PartialEq implementation
                  //     })),
                  //     SubClause::Always
                  // )]
            )
            .is_ok());

        assert!(app
            .mount(
                Id::ExecuteErrorResult,
                Box::new(ErrorResult::default()),
                vec![
                    Sub::new(
                        SubEventClause::User(TisqEvent::DbResponse(
                            // the content does not matter due to the PartialEq implementation
                            DbResponse::Error(Uuid::new_v4(), "".to_string()) // <- this is dummy
                        )),
                        SubClause::Always
                    ),
                    Sub::new(SubEventClause::WindowResize, SubClause::Always)
                ]
            )
            .is_ok());
    }

    fn mount_server_add_form(app: &mut TisqApplication) {
        // Mount inputs

        assert!(app
            .mount(
                Id::ServerNameInput,
                Box::new(InputText::new("Server Name", "")),
                vec![]
            )
            .is_ok());

        assert!(app
            .mount(
                Id::ConnectionUrlInput,
                Box::new(InputText::new("Connection URL", "")),
                vec![]
            )
            .is_ok());
    }

    fn unmount_server_add_form(&mut self) {
        assert!(self.app.umount(&Id::ServerNameInput).is_ok());
        assert!(self.app.umount(&Id::ConnectionUrlInput).is_ok());
    }

    // fn mount_add_server_form(app: &mut TisqApplication) {
    //     // Mount editor, subscribe to tick
    //     assert!(app
    //         .mount(
    //             Id::AddServerForm,
    //             Box::new(AddServerForm::default()),
    //             vec![]
    //         )
    //         .is_ok());
    // }

    fn init_app(
        storage: &storage::Storage,
        event_dispatcher: Box<EventDispatcherPort<TisqEvent>>,
    ) -> TisqApplication {
        // Setup application
        // NOTE: NoUserEvent is a shorthand to tell tui-realm we're not going to use any custom user event
        // NOTE: the event listener is configured to use the default crossterm input listener and to raise a Tick event each second
        // which we will use to update the clock

        let mut app: TisqApplication = Application::init(
            EventListenerCfg::default()
                .default_input_listener(Duration::from_millis(20))
                .port(event_dispatcher, Duration::from_millis(100))
                .poll_timeout(Duration::from_millis(10))
                .tick_interval(Duration::from_secs(1)),
        );

        // Mount components
        // assert!(app
        //     .mount(
        //         Id::Label,
        //         Box::new(
        //             Label::default()
        //                 .text("Waiting for a Msg...")
        //                 .alignment(Alignment::Left)
        //                 .background(Color::Reset)
        //                 .foreground(Color::LightYellow)
        //                 .modifiers(TextModifiers::BOLD),
        //         ),
        //         Vec::default(),
        //     )
        //     .is_ok());

        // Mount tree
        assert!(app
            .mount(
                Id::Tree,
                Box::new(BrowserTree::new(
                    Tree::new(
                        // Self::dir_tree(
                        // std::env::current_dir().ok().unwrap().as_path(),
                        // 3
                        // )
                        Self::browser_tree(storage).unwrap()
                    ),
                    Some("servers".to_string())
                )),
                vec![]
            )
            .is_ok());
        Self::mount_editor_envelope(&mut app);
        // Mount clock, subscribe to tick
        // assert!(app
        //     .mount(
        //         Id::Clock,
        //         Box::new(
        //             Clock::new(SystemTime::now())
        //                 .alignment(Alignment::Center)
        //                 .background(Color::Reset)
        //                 .foreground(Color::Cyan)
        //                 .modifiers(TextModifiers::BOLD)
        //         ),
        //         vec![Sub::new(SubEventClause::Tick, SubClause::Always)]
        //     )
        //     .is_ok());
        // Mount counters
        // assert!(app
        //     .mount(
        //         Id::LetterCounter,
        //         Box::new(LetterCounter::new(0)),
        //         Vec::new()
        //     )
        //     .is_ok());
        // assert!(app
        //     .mount(
        //         Id::DigitCounter,
        //         Box::new(DigitCounter::new(5)),
        //         Vec::default()
        //     )
        //     .is_ok());
        assert!(app
            .mount(
                Id::GlobalListener,
                Box::new(GlobalListener::default()),
                GlobalListener::subscriptions()
            )
            .is_ok());

        assert!(app
            .mount(
                Id::FormSubmitListener,
                Box::new(FormSubmitListener::default()),
                vec![FormSubmitListener::get_subscription()]
            )
            .is_ok());
        // Active letter counter
        // assert!(app.active(&Id::LetterCounter).is_ok());
        // assert!(app.active(&Id::Editor).is_ok());
        assert!(app.active(&Id::Tree).is_ok());
        app
    }

    fn get_or_set_shown_editor_id(&mut self) -> Option<Id> {
        if let Some(editor_id) = &self.shown_editor {
            return Some(Id::Editor(editor_id.clone()));
        }
        let first = self.query_editors.iter().next();
        let id = first.map(|(id, _)| Id::Editor(id.clone()));
        if let Some(Id::Editor(editor_id)) = id.clone() {
            self.shown_editor = Some(editor_id);
        }
        id
    }

    fn update_current_editor_tab(&mut self, editor_id: &EditorId) {
        let editor_index = self
            .query_editors
            .keys()
            .position(|some_id| some_id == editor_id);

        // if let Some(editor_index) = editor_index {
        let editor_index = editor_index.unwrap();
        // if let Some(editor_index) = editor_index {
        tracing::debug!("activating tab: {}", editor_index);
        self.app
            .attr(
                &Id::EditorTabs,
                Attribute::Custom(ACTIVE_TAB_INDEX),
                AttrValue::Length(editor_index),
            )
            .unwrap();
        // }
    }

    fn move_editor_tab(&mut self, editor_id: &EditorId, increment: i16) {
        let editor_index = self
            .query_editors
            .keys()
            .position(|some_id| some_id == editor_id);

        if let Some(editor_index) = editor_index {
            // let next_index = (editor_index as i16) + increment;
            // TODO: probably need to use a different data structure to
            // not have to pop all items and insert in different order

            tracing::debug!("moving from editor index: {}", editor_index);
            let to_pop = self.query_editors.len() - editor_index - 1;

            let mut popped = Vec::new();
            for _ in 0..to_pop {
                let (id, metadata) = self.query_editors.pop_back_entry().unwrap();
                popped.push((id, metadata));
            }

            tracing::debug!("popped: {}", popped.len());

            let (target_id, target_metadata) = self.query_editors.pop_back_entry().unwrap();

            if increment > 0 {
                if let Some((id, metadata)) = popped.pop() {
                    self.query_editors.insert(id, metadata);
                }
                self.query_editors.insert(target_id, target_metadata);
            } else if increment < 0 {
                if let Some((id, metadata)) = self.query_editors.pop_back_entry() {
                    tracing::debug!("inserting target and previous");
                    self.query_editors.insert(target_id, target_metadata);
                    self.query_editors.insert(id, metadata);
                } else {
                    tracing::debug!("inserting target only");
                    self.query_editors.insert(target_id, target_metadata);
                }
            } else {
                // this is not supposed to happen actually
                self.query_editors.insert(target_id, target_metadata);
            }

            for (id, metadata) in popped {
                self.query_editors.insert(id, metadata);
            }

            self.update_editor_tabs();
            self.update_current_editor_tab(editor_id);
        }
    }

    fn increment_editor(&mut self, increment: i16) {
        let current_editor_index = match &self.shown_editor {
            Some(shown_editor) => self
                .query_editors
                .keys()
                .position(|some_id| some_id == shown_editor),
            None => None,
        };

        if let Some(current_editor_index) = current_editor_index {
            let next_index = (current_editor_index as i16) + increment;

            let next_index = if next_index < 0 {
                self.query_editors.len() - 1
            } else if next_index as usize >= self.query_editors.len() {
                0
            } else {
                next_index as usize
            };

            let next_editor_id = self
                .query_editors
                .keys()
                .nth(next_index)
                .expect("next editor not found");

            self.shown_editor = Some(next_editor_id.clone());
            self.app
                .active(&Id::Editor(next_editor_id.clone()))
                .unwrap();
            self.update_current_editor_tab(&next_editor_id.clone());
        }
    }

    fn update_editor_tabs(&mut self) {
        self.app
            .attr(
                &Id::EditorTabs,
                Attribute::Value,
                AttrValue::Payload(PropPayload::Vec(
                    self.query_editors
                        .iter()
                        .map(
                            |(EditorId { database, .. }, EditorMetadata { name: server_name })| {
                                PropValue::TextSpan(TextSpan::new(format!(
                                    "{}/{}",
                                    server_name, database
                                )))
                            },
                        )
                        .collect(),
                )),
            )
            .unwrap();
    }
}

// Let's implement Update for model

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        if let Some(msg) = msg {
            // Set redraw
            self.redraw = true;
            // Match message
            match msg {
                Msg::NavigateRight | Msg::NavigateLeft => match self.app.focus() {
                    Some(&Id::Tree) => Some(Msg::ChangeFocus(Id::EditorPanel)),
                    Some(&Id::Editor(_)) => Some(Msg::ChangeFocus(Id::Tree)),
                    _ => None,
                },
                Msg::ShowErrorResult => {
                    self.execute_result_state = ExecuteResultState::Error;
                    None
                }
                Msg::ShowFetchedTable => {
                    self.execute_result_state = ExecuteResultState::FetchedTable;
                    None
                }
                Msg::MoveTabRight(editor_id) => {
                    tracing::debug!("moving tab right");
                    self.move_editor_tab(&editor_id, 1);
                    None
                }
                Msg::MoveTabLeft(editor_id) => {
                    tracing::debug!("moving tab left");
                    self.move_editor_tab(&editor_id, -1);
                    None
                }
                Msg::NextEditor => {
                    self.increment_editor(1);
                    None
                }
                Msg::PreviousEditor => {
                    self.increment_editor(-1);
                    None
                }
                Msg::OpenQueryEditor(server_id, database) => {
                    let server = self.storage.get_server(server_id).unwrap().unwrap();
                    let server_name = server.name;

                    let editor_id = EditorId {
                        server_id,
                        database: database.clone(),
                    };

                    if self.query_editors.contains_key(&editor_id) {
                        // TODO: activate that tab

                        self.app.active(&Id::Editor(editor_id.clone())).unwrap();
                        self.shown_editor = Some(editor_id.clone());
                        self.update_current_editor_tab(&editor_id);

                        return None;
                    }
                    self.mount_editor(editor_id.clone());

                    self.app.active(&Id::Editor(editor_id.clone())).unwrap();
                    self.shown_editor = Some(editor_id.clone());

                    self.query_editors
                        .insert(editor_id.clone(), EditorMetadata { name: server_name });

                    self.update_editor_tabs();

                    self.update_current_editor_tab(&editor_id);

                    // This is to make sure we have an active connection
                    // TODO: process response as well
                    let props = server.connection_properties;
                    let connection_url =
                        props.get("url").expect("connection url not found").clone();
                    self.connection_manager_tx
                        .send(DbRequest::ConnectToDatabase(
                            server_id,
                            database,
                            connection_url,
                        ))
                        .unwrap();

                    None
                }
                Msg::OpenConnection(server_id) => {
                    let server = self.storage.get_server(server_id).unwrap();
                    let server = server.unwrap();
                    let props = server.connection_properties;
                    let connection_url =
                        props.get("url").expect("connection url not found").clone();

                    self.connection_manager_tx
                        .send(DbRequest::ConnectToServer(server_id, connection_url))
                        .unwrap();

                    // let connection: eyre::Result<Connection> =
                    //     task::block_on(task::spawn(async move {
                    //         let connection =
                    //             Connection::connect(server.name, connection_url).await?;
                    //         Ok(connection)
                    //     }));

                    // let connection = connection.unwrap();
                    // self.connections.insert(server_id, connection);

                    Some(Msg::LoadDatabases(server_id))
                }
                Msg::LoadDatabases(server_id) => {
                    // let connection = self.connections.get_mut(&server_id).unwrap();

                    // let databases: eyre::Result<Vec<String>> =
                    //     task::block_on(task::spawn(async move {
                    //         let databases = connection.list_databases().await?;
                    //         // Ok(databases)
                    //         Ok(vec!["test".to_string(), "test2".to_string()])
                    //     }));
                    self.connection_manager_tx
                        .send(DbRequest::ListDatabases(server_id))
                        .unwrap();

                    None
                }
                Msg::DeleteBrowsedNode(id) => {
                    let mut split = id.split(':');
                    let group = match split.next() {
                        Some(group) => group,
                        None => return None,
                    };
                    if group != "server" {
                        return None;
                    }
                    let server_id = split.next().unwrap();
                    let server_id = match Uuid::parse_str(server_id) {
                        Ok(server_id) => server_id,
                        Err(_) => {
                            tracing::error!("invalid server id: {}", server_id);
                            return None;
                        }
                    };
                    self.storage.delete_server(server_id).unwrap();
                    self.update_browser();
                    None
                }
                Msg::SubmitAddServerForm => {
                    let server_name_state = self.app.state(&Id::ServerNameInput).unwrap();
                    let server_name = match server_name_state {
                        State::One(StateValue::String(input_text)) => input_text,
                        _ => {
                            panic!("unexpected state: {:?}", server_name_state);
                        }
                    };

                    let connection_url_state = self.app.state(&Id::ConnectionUrlInput).unwrap();
                    let connection_url = match connection_url_state {
                        State::One(StateValue::String(input_text)) => input_text,
                        _ => {
                            panic!("unexpected state: {:?}", connection_url_state);
                        }
                    };

                    self.storage
                        .add_server(NewServer {
                            connection_properties: HashMap::from([(
                                "url".to_string(),
                                connection_url,
                            )]),
                            name: server_name,
                        })
                        .unwrap();

                    self.update_browser();

                    self.app.active(&Id::Tree).unwrap();
                    self.unmount_server_add_form();
                    self.add_server_form_mounted = false;
                    self.adding_server = false;

                    None
                }
                Msg::FocusPreviousInput => {
                    self.add_server_form
                        .activate_previous_input(&mut self.app)
                        .unwrap();
                    None
                }
                Msg::FocusNextInput => {
                    self.add_server_form
                        .activate_next_input(&mut self.app)
                        .unwrap();
                    None
                }
                Msg::StartAddingServer => {
                    self.adding_server = true;
                    if !self.add_server_form_mounted {
                        Self::mount_server_add_form(&mut self.app);

                        // Self::mount_add_server_form(&mut self.app);
                        self.add_server_form_mounted = true;
                        // self.app.active(&Id::AddServerForm).unwrap();

                        self.app.active(&Id::ServerNameInput).unwrap();
                    }

                    // self.storage
                    //     .add_server(NewServer {
                    //         connection_properties: HashMap::new(),
                    //         name: "test server".to_string(),
                    //     })
                    //     .unwrap();

                    // self.update_browser();

                    // self.app.umount(&Id::Editor).unwrap();

                    // Self::mount_editor(&mut self.app);

                    // self.app.attr(
                    // &Id::Tree,
                    // Attribute::Custom(TREE_PROPERTY),
                    // AttrValue::Payload(PropPayload::One()),
                    // );
                    // self.app.
                    None
                }
                Msg::ExecuteQuery(editor_id, query) => {
                    // let execute_result = execute_query(query);
                    // self.event_dispatcher_port.dispatch(Event::User(
                    //     TisqEvent::QueryResultFetched(QueryResult {
                    //         data: execute_result,
                    //     }),
                    // ));
                    self.connection_manager_tx
                        .send(DbRequest::Execute(
                            // self.get_or_set_shown_editor_id().unwrap(),
                            editor_id.server_id,
                            editor_id.database,
                            query,
                        ))
                        .unwrap();

                    // println!("got execute result: {:?}", execute_result);
                    // return Some(Msg::QueryResultFetched(QueryResult {
                    // data: execute_result,
                    // }));
                    None
                }
                Msg::AppClose => {
                    self.quit = true; // Terminate
                    None
                }
                Msg::ChangeFocus(id) => {
                    if id == Id::EditorPanel {
                        // let first = self.query_editors.iter().next();
                        // let id = first.map(|(id, metadata)| Id::Editor(id.clone()));
                        let id = self.get_or_set_shown_editor_id();
                        if let Some(id) = id {
                            assert!(self.app.active(&id).is_ok());
                        }

                        return None;
                    }
                    assert!(self.app.active(&id).is_ok());
                    None
                }
                Msg::None => {
                    // todo!()
                    None
                }
            }
        } else {
            None
        }
    }
}
