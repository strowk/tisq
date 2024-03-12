use core::fmt;
use std::{collections::HashMap, hash::Hash, vec};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use tuirealm::{
    event::{Key, KeyEvent, KeyEventKind, KeyModifiers},
    Sub, SubEventClause,
};

use super::KeySubClause;

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct KeyPressConfig {
    key: Key,
    modifiers: KeyModifiersConfig,
}

#[derive(Clone)]
pub(crate) enum KeyModifiersConfig {
    TuiModifiers(KeyModifiers),
}

impl Serialize for KeyModifiersConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let modifiers = match self {
            KeyModifiersConfig::TuiModifiers(modifiers) => modifiers,
        };
        let result = key_modifiers_to_string(modifiers);
        serializer.serialize_str(&result)
    }
}

fn key_modifiers_to_string(modifiers: &KeyModifiers) -> String {
    let mut result: String = "".to_string();
    let mut has_one = false;
    if modifiers.intersects(KeyModifiers::CONTROL) {
        result = result + "Ctrl";
        has_one = true
    }
    if modifiers.intersects(KeyModifiers::ALT) {
        if has_one {
            result = result + "+";
        }
        result = result + "Alt";
        has_one = true;
    }
    if modifiers.intersects(KeyModifiers::SHIFT) {
        if has_one {
            result = result + "+";
        }
        result = result + "Shift";
    }
    result
}

struct KeyModifiersConfigVisitor;

impl<'de> Visitor<'de> for KeyModifiersConfigVisitor {
    type Value = KeyModifiersConfig;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            &"Plus '+' separated string of key modifiers Ctrl, Alt and Shift, 
            f.e Ctrl+Alt+Shift or Ctrl+Alt or Ctrl or Alt or Shift"
                .replace("\n", ""),
        )
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(KeyModifiersConfig::TuiModifiers(KeyModifiers::NONE))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let items = value.split("+");

        let mut modifiers = KeyModifiers::NONE;

        for item in items {
            match item {
                "Ctrl" => modifiers = modifiers | KeyModifiers::CONTROL,
                "Alt" => modifiers = modifiers | KeyModifiers::ALT,
                "Shift" => modifiers = modifiers | KeyModifiers::SHIFT,
                "" => continue,
                _ => return Err(E::custom(format!("Unknown key modifier: {}", item))),
            }
        }

        Ok(KeyModifiersConfig::TuiModifiers(modifiers))
    }
}

impl<'de> Deserialize<'de> for KeyModifiersConfig {
    fn deserialize<D>(deserializer: D) -> Result<KeyModifiersConfig, D::Error>
    where
        D: Deserializer<'de>,
    {
        let modifiers = deserializer.deserialize_str(KeyModifiersConfigVisitor)?;
        Ok(modifiers)
    }
}

pub(crate) type KeybindingSectionConfig<A> = HashMap<A, Vec<KeyPressConfig>>;

pub(crate) type KeybindingsConfig<A> = HashMap<String, KeybindingSectionConfig<A>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct KeybindingKeyPress {
    pub key: Key,
    pub modifiers: KeyModifiers,
}

impl fmt::Display for KeybindingKeyPress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_key = if let Key::Char(key) = self.key {
            format!("{}", key.to_uppercase())
        } else {
            format!("{:?}", self.key)
        };
        let key_modifiers = key_modifiers_to_string(&self.modifiers);
        if key_modifiers.is_empty() {
            return write!(f, "{}", formatted_key);
        }
        write!(f, "{}+{}", key_modifiers, formatted_key)
    }
}

impl From<&KeyPressConfig> for KeybindingKeyPress {
    fn from(key_press_config: &KeyPressConfig) -> Self {
        Self {
            key: key_press_config.key,
            modifiers: match key_press_config.modifiers {
                KeyModifiersConfig::TuiModifiers(modifiers) => modifiers,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SectionKeybindings<A> {
    pub actions: HashMap<A, Vec<KeybindingKeyPress>>,
    key_to_action: HashMap<KeybindingKeyPress, A>,
}

pub(crate) trait KeyboundAction {
    fn sections() -> Vec<&'static str>;
    fn list(section: &str) -> Vec<&Self>;
    fn get_default_bindings(&self) -> Vec<KeybindingKeyPress>;
}

impl<A> SectionKeybindings<A>
where
    A: KeyboundAction + Eq + PartialEq + Clone + Hash + std::fmt::Debug,
{
    fn get_bindings_mut(&mut self, action: &A) -> Option<&mut Vec<KeybindingKeyPress>> {
        self.actions.get_mut(action)
    }

    pub fn get_action(&self, key_event: &KeyEvent) -> Option<&A> {
        if key_event.kind != KeyEventKind::Press {
            return None;
        }
        self.key_to_action.get(&KeybindingKeyPress {
            key: key_event.code,
            modifiers: key_event.modifiers,
        })
    }

    pub fn subscriptions<K, E>(&self, sub_clause: KeySubClause<K>) -> Vec<Sub<K, E>>
    where
        K: Eq + PartialEq + Clone + Hash + std::fmt::Debug,
        E: Eq + PartialEq + Clone + PartialOrd,
    {
        let result: Vec<Sub<K, E>> = self
            .actions
            .values()
            .map(|keybindings| {
                keybindings
                    .iter()
                    .map(|keybinding| {
                        Sub::new(
                            SubEventClause::Keyboard(KeyEvent {
                                code: keybinding.key,
                                modifiers: keybinding.modifiers,
                                kind: KeyEventKind::Press,
                            }),
                            sub_clause.get_tui_realm_sub_clause(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        tracing::debug!("keybindings subscriptions n: {:?}", result.len());
        result
    }
}

pub(crate) struct Keybindings<A> {
    pub by_section: HashMap<String, SectionKeybindings<A>>,
    // pub global: SectionKeybindings<A>,
}

impl<A> Keybindings<A>
where
    A: KeyboundAction + Eq + PartialEq + Clone + Hash + std::fmt::Debug + Serialize,
{
    pub fn new(config: &KeybindingsConfig<A>) -> Self {
        let mut by_section = HashMap::new();

        tracing::debug!(
            "reading keybindings config: {}",
            toml::to_string(&config).unwrap()
        );

        let sections = A::sections();

        for section_name in sections {
            let mut section: SectionKeybindings<A> = SectionKeybindings {
                actions: HashMap::new(),
                key_to_action: HashMap::new(),
            };

            A::list(section_name).into_iter().for_each(|action| {
                section.actions.insert(action.clone(), vec![]);
            });

            let keybindings_by_action: Vec<(A, Vec<KeybindingKeyPress>)> = A::list(section_name)
                .into_iter()
                .map(|action| {
                    (
                        action.clone(),
                        match config.get(section_name) {
                            None => action.get_default_bindings(),
                            Some(section_config) => match section_config.get(action) {
                                None => action.get_default_bindings(),
                                Some(keybindings) => keybindings.iter().map(Into::into).collect(),
                            },
                        },
                    )
                })
                .collect();

            for (action, keybindings) in keybindings_by_action {
                for press in keybindings {
                    let kbs = match section.get_bindings_mut(&action) {
                        Some(kbs) => kbs,
                        None => continue,
                    };
                    kbs.push(press.clone());
                    section.key_to_action.insert(press, action.clone());
                }
            }
            by_section.insert(section_name.to_string(), section);
        }

        tracing::debug!("read keybindings config: {:?}", &by_section);
        Self { by_section }
    }
}
