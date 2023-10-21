use std::hash::Hash;

use tuirealm::{AttrValue, Attribute, State, SubClause};

// This is a workaround for missing Clone derive for SubClause
pub(crate) enum KeySubClause<K>
where
    K: Eq + PartialEq + Clone + Hash,
{
    /// Always forward event to component
    Always,
    /// Forward event if target component has provided attribute with the provided value
    /// If the attribute doesn't exist on component, result is always `false`.
    HasAttrValue(K, Attribute, AttrValue),
    /// Forward event if target component has provided state
    HasState(K, State),
    /// Forward event if target component is mounted
    IsMounted(K),
    /// Forward event if the inner clause is `false`
    Not(Box<KeySubClause<K>>),
    /// Forward event if both the inner clauses are `true`
    And(Box<KeySubClause<K>>, Box<KeySubClause<K>>),
    /// Forward event if at least one of the inner clauses is `true`
    Or(Box<KeySubClause<K>>, Box<KeySubClause<K>>),
}

impl<K> KeySubClause<K>
where
    K: Eq + PartialEq + Clone + Hash,
{
    pub(super) fn get_tui_realm_sub_clause(&self) -> SubClause<K> {
        match self {
            KeySubClause::Always => SubClause::Always,
            KeySubClause::HasAttrValue(id, attr, value) => {
                SubClause::HasAttrValue(id.clone(), attr.clone(), value.clone())
            }
            KeySubClause::HasState(id, state) => SubClause::HasState(id.clone(), state.clone()),
            KeySubClause::IsMounted(id) => SubClause::IsMounted(id.clone()),
            KeySubClause::Not(sub_clause) => {
                SubClause::Not(Box::new(sub_clause.get_tui_realm_sub_clause()))
            }
            KeySubClause::And(sub_clause1, sub_clause2) => SubClause::And(
                Box::new(sub_clause1.get_tui_realm_sub_clause()),
                Box::new(sub_clause2.get_tui_realm_sub_clause()),
            ),
            KeySubClause::Or(sub_clause1, sub_clause2) => SubClause::Or(
                Box::new(sub_clause1.get_tui_realm_sub_clause()),
                Box::new(sub_clause2.get_tui_realm_sub_clause()),
            ),
        }
    }
}
