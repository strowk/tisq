use std::vec;

use tuirealm::{
    command::{Cmd, CmdResult},
    props::{PropPayload, PropValue, Style, TextSpan},
    tui::{
        text::{Line, Span},
        widgets::Tabs,
    },
    AttrValue, Attribute, Component, Event, MockComponent, Props, State, StateValue,
};

use crate::{app::TisqEvent, Msg};

pub const ACTIVE_TAB_INDEX: &str = "active-tab-index";

pub struct EditorTabsState {
    active_tab_index: usize,
}

impl From<&EditorTabsState> for State {
    fn from(s: &EditorTabsState) -> Self {
        State::One(StateValue::Usize(s.active_tab_index))
    }
}

pub struct EditorTabs<'a> {
    props: Props,
    state: EditorTabsState,
    tabs: Vec<TabTitle>,
    widget: Tabs<'a>,
}

pub struct TabTitle {
    title: TextSpan,
}

impl From<&TabTitle> for Line<'_> {
    fn from(t: &TabTitle) -> Self {
        // TODO: add style from text span
        Line::from(Span::raw(t.title.content.clone()))
    }
}

impl<'a> MockComponent for EditorTabs<'_> {
    fn view(&mut self, frame: &mut tuirealm::Frame, area: tuirealm::tui::layout::Rect) {
        frame.render_widget(self.widget.clone(), area);
    }

    fn query(&self, query: Attribute) -> Option<AttrValue> {
        self.props.get(query)
    }

    fn attr(&mut self, query: Attribute, value: AttrValue) {
        self.props.set(query, value.clone());
        match query {
            Attribute::Value => {
                if let AttrValue::Payload(PropPayload::Vec(values)) = value {
                    self.tabs = vec![];
                    values.iter().for_each(|v| {
                        if let PropValue::TextSpan(s) = v {
                            self.tabs.push(TabTitle { title: s.clone() });
                        }
                    });
                    self.widget = Self::new_widget(self.tabs.iter().collect());
                }
            }
            Attribute::Custom(ACTIVE_TAB_INDEX) => {
                if let AttrValue::Length(index) = value {
                    self.state.active_tab_index = index;
                    self.widget = self.widget.clone().select(index);
                }
            }
            _ => {}
        }
    }

    fn state(&self) -> State {
        (&self.state).into()
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl EditorTabs<'_> {
    pub fn new() -> Self {
        Self {
            props: Props::default(),
            state: EditorTabsState {
                active_tab_index: 0,
            },
            tabs: vec![],
            widget: Self::new_widget(vec![]),
        }
    }

    fn new_widget(titles: Vec<&TabTitle>) -> Tabs<'static> {
        Tabs::new::<&TabTitle>(titles).highlight_style(
            Style::default()
                .bg(tuirealm::props::Color::Cyan)
                .fg(tuirealm::props::Color::Black)
                .add_modifier(tuirealm::props::TextModifiers::UNDERLINED),
        )
    }
}

impl Component<Msg, TisqEvent> for EditorTabs<'_> {
    fn on(&mut self, _ev: Event<TisqEvent>) -> Option<Msg> {
        // Some(Msg::None)
        None
    }
}
