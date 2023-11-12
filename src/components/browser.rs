use tui_realm_treeview::{Node, Tree, TreeView, TREE_CMD_CLOSE, TREE_CMD_OPEN};
use tuirealm::{
    command::{Cmd, Direction, Position},
    event::{Key, KeyEvent, KeyEventKind, KeyModifiers},
    props::{Alignment, BorderSides, BorderType, Borders, Color, Style},
    Component, Event, MockComponent, State, StateValue,
};
use uuid::Uuid;

use crate::{
    app::{DbResponse, SectionKeybindings, TisqEvent, TisqKeyboundAction},
    Msg,
};

pub enum BrowserTreeId {
    Server(Uuid),
    Database(Uuid, String),
    Schema(Uuid, String, String),
    Table(Uuid, String, String, String),
    Column {
        server_id: Uuid,
        database: String,
        schema: String,
        table: String,
        column: String,
    },
}

impl BrowserTreeId {
    fn parse_server_uuid(s: &str) -> Option<Uuid> {
        match Uuid::parse_str(s) {
            Ok(uuid) => Some(uuid),
            Err(_) => {
                tracing::error!("Could not parse server id: {}", s);
                None
            }
        }
    }
    fn parse_str(s: &str) -> Option<Self> {
        let mut parts = s.splitn(2, ':');
        let section = parts.next()?;

        match section {
            "server" => {
                let node: &str = parts.next()?;
                let server_id = Self::parse_server_uuid(node)?;
                Some(Self::Server(server_id))
            }
            "database" => {
                let node: &str = parts.next()?;
                let mut parts = node.splitn(2, ':');
                let server_id = parts.next()?;
                let server_id = Self::parse_server_uuid(server_id)?;
                Some(Self::Database(server_id, parts.next()?.to_string()))
            }
            "schema" => {
                let node: &str = parts.next()?;
                let mut parts = node.splitn(3, ':');
                let server_id = parts.next()?;
                let server_id = Self::parse_server_uuid(server_id)?;
                Some(Self::Schema(
                    server_id,
                    parts.next()?.to_string(),
                    parts.next()?.to_string(),
                ))
            }
            "table" => {
                let node: &str = parts.next()?;
                let mut parts = node.splitn(4, ':');
                let server_id = parts.next()?;
                let server_id = Self::parse_server_uuid(server_id)?;
                Some(Self::Table(
                    server_id,
                    parts.next()?.to_string(),
                    parts.next()?.to_string(),
                    parts.next()?.to_string(),
                ))
            }
            "column" => {
                let node: &str = parts.next()?;
                let mut parts = node.splitn(4, ':');
                let server_id = parts.next()?;
                let server_id = Self::parse_server_uuid(server_id)?;
                Some(Self::Column {
                    server_id,
                    database: parts.next()?.to_string(),
                    schema: parts.next()?.to_string(),
                    table: parts.next()?.to_string(),
                    column: parts.next()?.to_string(),
                })
            }
            _ => None,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Server(id) => format!("server:{}", id),
            Self::Database(server_id, name) => format!("database:{}:{}", server_id, name),
            Self::Schema(server_id, database, name) => {
                format!("schema:{}:{}:{}", server_id, database, name)
            }
            Self::Table(server_id, database, schema, table) => {
                format!("table:{}:{}:{}:{}", server_id, database, schema, table)
            }
            Self::Column {
                server_id,
                database,
                schema,
                table,
                column,
            } => {
                format!(
                    "column:{}:{}:{}:{}:{}",
                    server_id, database, schema, table, column
                )
            }
        }
    }
}

#[derive(MockComponent)]
pub(crate) struct BrowserTree {
    component: TreeView,
    keybindings: SectionKeybindings<TisqKeyboundAction>,
}

impl BrowserTree {
    pub fn set_tree(&mut self, tree: Tree) {
        // tree.root().
        // self.component.preserve_state(true);
        // tree.root_mut().op
        self.component.set_tree(tree);
    }

    pub fn new(
        tree: Tree,
        initial_node: Option<String>,
        keybindings: SectionKeybindings<TisqKeyboundAction>,
    ) -> Self {
        // Preserve initial node if exists
        let initial_node = match initial_node {
            Some(id) if tree.root().query(&id).is_some() => id,
            _ => tree.root().id().to_string(),
        };
        BrowserTree {
            keybindings,
            component: TreeView::default()
                .foreground(Color::Reset)
                .borders(
                    Borders::default()
                        .color(Color::LightYellow)
                        .sides(BorderSides::NONE)
                        .modifiers(BorderType::Rounded),
                )
                .inactive(Style::default().fg(Color::Gray))
                .indent_size(3)
                .scroll_step(6)
                .title("Db Browser", Alignment::Center)
                .highlighted_color(Color::LightYellow)
                .highlight_symbol("🦄")
                .with_tree(tree)
                .preserve_state(true)
                .initial_node(initial_node),
        }
    }

    fn open_query_editor(&self) -> Option<Msg> {
        let selected_id = self.component.tree_state().selected();
        let selected_id = match selected_id {
            Some(id) => id,
            None => return Some(Msg::None),
        };
        let database = match BrowserTreeId::parse_str(&selected_id) {
            Some(BrowserTreeId::Database(_server_id, database)) => database,
            _ => return Some(Msg::None),
        };

        if let Some(parent_node) = self
            .component
            .tree()
            .root()
            .parent(&selected_id.to_string())
        {
            let server_id = match BrowserTreeId::parse_str(parent_node.id()) {
                Some(BrowserTreeId::Server(server_id)) => server_id,
                _ => return Some(Msg::None),
            };
            return Some(Msg::OpenQueryEditor(server_id, database));
        }
        return Some(Msg::None);
    }
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub(crate) struct SentTree(pub Tree);

impl PartialOrd for SentTree {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.root().id().cmp(other.0.root().id()))
    }
}

impl Component<Msg, TisqEvent> for BrowserTree {
    fn on(&mut self, ev: Event<TisqEvent>) -> Option<Msg> {
        let res_message = match ev {
            Event::Keyboard(kb_event) => match self.keybindings.get_action(&kb_event) {
                Some(TisqKeyboundAction::BrowserAddServer) => Some(Msg::StartAddingServer),
                Some(TisqKeyboundAction::BrowserDatabaseOpenQueryEditor) => {
                    self.open_query_editor()
                }
                _ => None,
            },
            _ => None,
        };
        if let Some(msg) = res_message {
            return Some(msg);
        }
        let _result = match ev {
            Event::User(TisqEvent::TreeReloaded(SentTree(tree))) => {
                self.set_tree(tree);
                return Some(Msg::None);
            }

            Event::User(TisqEvent::DbResponse(DbResponse::ColumnsListed {
                server_id,
                database,
                schema,
                table,
                columns,
            })) => {
                // tracing::debug!("Schemas listed: {:?}", tables);
                let table_id: BrowserTreeId = BrowserTreeId::Table(
                    server_id,
                    database.clone(),
                    schema.clone(),
                    table.clone(),
                );
                let tree_id = table_id.to_string();
                {
                    let tree = self.component.tree_mut();
                    let node = tree.root_mut().query_mut(&tree_id)?;
                    node.clear();
                    for column in columns {
                        let id = BrowserTreeId::Column {
                            server_id,
                            database: database.clone(),
                            schema: schema.clone(),
                            table: table.clone(),
                            column: column.clone(),
                        }
                        .to_string();
                        let column_node = Node::new(id, column.clone());

                        // dummy is created to make the table node expandable
                        // it is a workaround for the limitation of the treeview component
                        // let dummy: Node = Node::new(
                        //     format!("dummy:{}/{}/{}/{}", server_id, database, schema),
                        //     "loading tables...".to_string(),
                        // );
                        // table_node.add_child(dummy);

                        node.add_child(column_node);
                    }
                }
                // This solution can work as well without blocking, but requires
                // open access to open_node method as well as new one tree_state_mut
                let node = self.component.tree().root().query(&tree_id)?.clone();
                let root = self.component.tree().root().clone();
                self.component.tree_state_mut().open_node(&root, &node);

                return Some(Msg::None);
            }

            Event::User(TisqEvent::DbResponse(DbResponse::TablesListed {
                server_id,
                database,
                schema,
                tables,
            })) => {
                // tracing::debug!("Schemas listed: {:?}", tables);
                let schema_id = BrowserTreeId::Schema(server_id, database.clone(), schema.clone());
                let tree_id = schema_id.to_string();
                {
                    let tree = self.component.tree_mut();
                    let node = tree.root_mut().query_mut(&tree_id)?;
                    node.clear();
                    for table in tables {
                        let id = BrowserTreeId::Table(
                            server_id,
                            database.clone(),
                            schema.clone(),
                            table.clone(),
                        )
                        .to_string();
                        let mut table_node = Node::new(id, table.clone());

                        // dummy is created to make the table node expandable
                        // it is a workaround for the limitation of the treeview component
                        let dummy: Node = Node::new(
                            format!("dummy:{}/{}/{}/{}", server_id, database, schema, table),
                            "loading tables...".to_string(),
                        );
                        table_node.add_child(dummy);

                        node.add_child(table_node);
                    }
                }
                // This solution can work as well without blocking, but requires
                // open access to open_node method as well as new one tree_state_mut
                let node = self.component.tree().root().query(&tree_id)?.clone();
                let root = self.component.tree().root().clone();
                self.component.tree_state_mut().open_node(&root, &node);

                return Some(Msg::None);
            }

            Event::User(TisqEvent::DbResponse(DbResponse::SchemasListed {
                server_id,
                database,
                schemas,
            })) => {
                // tracing::debug!("Schemas listed: {:?}", tables);
                {
                    let tree = self.component.tree_mut();
                    let tree_id = BrowserTreeId::Database(server_id, database.clone()).to_string();
                    let node = tree.root_mut().query_mut(&tree_id)?;
                    node.clear();
                    for schema in schemas {
                        let id = BrowserTreeId::Schema(server_id, database.clone(), schema.clone())
                            .to_string();
                        let mut schema_node = Node::new(id, schema.clone());

                        // dummy is created to make the schema node expandable
                        // it is a workaround for the limitation of the treeview component
                        let dummy: Node = Node::new(
                            format!("dummy:{}/{}/{}", server_id, database, schema),
                            "loading tables...".to_string(),
                        );
                        schema_node.add_child(dummy);

                        node.add_child(schema_node);
                    }
                }
                // This solution can work as well without blocking, but requires
                // open access to open_node method as well as new one tree_state_mut
                let tree_id = BrowserTreeId::Database(server_id, database.clone());
                let node = self
                    .component
                    .tree()
                    .root()
                    .query(&tree_id.to_string())?
                    .clone();
                let root = self.component.tree().root().clone();
                self.component.tree_state_mut().open_node(&root, &node);

                return Some(Msg::None);
            }
            Event::User(TisqEvent::DbResponse(DbResponse::DatabasesListed(
                server_id,
                databases,
            ))) => {
                // tracing::debug!("Databases listed: {:?}", databases);
                {
                    let tree = self.component.tree_mut();
                    let tree_id = BrowserTreeId::Server(server_id);
                    let node = tree.root_mut().query_mut(&tree_id.to_string())?;
                    node.clear();
                    for database in databases {
                        let id = BrowserTreeId::Database(server_id, database.clone()).to_string();
                        let mut database_node = Node::new(id, database.clone());

                        // dummy is created to make the server node expandable
                        // it is a workaround for the limitation of the treeview component
                        let dummy: Node = Node::new(
                            format!("dummy:{}/{}", server_id, database),
                            "loading schemas...".to_string(),
                        );
                        database_node.add_child(dummy);

                        node.add_child(database_node);
                    }
                }

                // this can potentially not work if user has switched to another node
                // in the meantime. To avoid this, we should block switching to another node
                // while database is being opened, however that can have other side effects

                // let tree_id = BrowserTreeId::Server(server_id.to_string());
                // let node = self
                //     .component
                //     .tree()
                //     .root()
                //     .query(&tree_id.to_string())?
                //     .clone();
                // self.component.tree_state_mut().open(&node);

                // if we do block switching to another node, then we can do this insead:
                // self.perform(Cmd::Custom(TREE_CMD_OPEN))

                // This solution can work as well without blocking, but requires
                // open access to open_node method as well as new one tree_state_mut
                let tree_id = BrowserTreeId::Server(server_id);
                let node = self
                    .component
                    .tree()
                    .root()
                    .query(&tree_id.to_string())?
                    .clone();
                let root = self.component.tree().root().clone();
                self.component.tree_state_mut().open_node(&root, &node);

                return Some(Msg::None);
            }
            // Event::Keyboard(KeyEvent {
            //     code: Key::Char('q'),
            //     kind: KeyEventKind::Press,
            //     modifiers: KeyModifiers::NONE,
            // }) => return self.open_query_editor(),
            // Event::Keyboard(KeyEvent {
            //     code: Key::Char('a'),
            //     kind: KeyEventKind::Press,
            //     modifiers: KeyModifiers::NONE,
            // }) => return Some(Msg::StartAddingServer),
            // Event::Keyboard(KeyEvent {
            //     code: Key::Left | Key::Right,
            //     kind: KeyEventKind::Press,
            //     modifiers: KeyModifiers::ALT,
            // }) => return Some(Msg::ChangeFocus(Id::EditorPanel)),
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => self.perform(Cmd::Custom(TREE_CMD_CLOSE)),
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => {
                match self.component.state() {
                    // if node is server, open connection
                    State::One(StateValue::String(id)) => match BrowserTreeId::parse_str(&id) {
                        Some(BrowserTreeId::Server(server_id)) => {
                            return Some(Msg::OpenConnection(server_id))
                        }
                        Some(BrowserTreeId::Database(server_id, database)) => {
                            return Some(Msg::OpenDatabase(server_id, database, 0))
                        }
                        Some(BrowserTreeId::Schema(server_id, database, schema)) => {
                            return Some(Msg::OpenSchema {
                                server_id,
                                database,
                                schema,
                                retries: 0,
                            })
                        }
                        Some(BrowserTreeId::Table(server_id, database, schema, table)) => {
                            return Some(Msg::OpenTable {
                                server_id,
                                database,
                                schema,
                                table,
                                retries: 0,
                            })
                        }
                        _ => self.perform(Cmd::Custom(TREE_CMD_OPEN)),
                    },
                    // open tree otherwise
                    _ => self.perform(Cmd::Custom(TREE_CMD_OPEN)),
                }
            }
            Event::Keyboard(KeyEvent {
                code: Key::PageDown,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::PageUp,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => self.perform(Cmd::Move(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Home,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent {
                code: Key::End,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => self.perform(Cmd::GoTo(Position::End)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => self.perform(Cmd::Submit),
            Event::Keyboard(KeyEvent {
                code: Key::Delete,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
            }) => match self.component.state() {
                State::One(StateValue::String(id)) => {
                    return Some(Msg::DeleteBrowsedNode(id));
                }
                _ => return None,
            },
            // Event::Keyboard(KeyEvent {
            //     code: Key::Esc,
            //     modifiers: KeyModifiers::NONE,
            //     kind: KeyEventKind::Press
            // }) => return Some(Msg::AppClose),
            // Event::Keyboard(KeyEvent {
            //     code: Key::Backspace,
            //     modifiers: KeyModifiers::NONE,
            //     kind: KeyEventKind::Press,
            // }) => return Some(Msg::GoToUpperDir),
            // Event::Keyboard(KeyEvent {
            //     code: Key::Tab,
            //     modifiers: KeyModifiers::NONE,
            // }) => return Some(Msg::FsTreeBlur),
            _ => return None,
        };
        Some(Msg::None)
        // match result {
        // CmdResult::Submit(State::One(StateValue::String(node))) => Some(Msg::ExtendDir(node)),
        // _ => Some(Msg::None),
        // }
    }
}
