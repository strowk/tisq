use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct Snippet {
    pub shortcut: String,
    pub description: String,
    pub query: String,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash)]
pub(crate) enum SnippetDatabase {
    Postgres,
}

pub(crate) type SnippetsConfig = HashMap<SnippetDatabase, Vec<Snippet>>;

pub(crate) fn standard_postgres_snippets() -> HashMap<String, Snippet> {
    HashMap::from([
        (
            "sel".to_string(),
            Snippet {
                shortcut: "sel".to_string(),
                description: "SELECT * FROM".to_string(),
                query: "SELECT * FROM".to_string(),
            },
        ),
        (
            "ins".to_string(),
            Snippet {
                shortcut: "ins".to_string(),
                description: "INSERT INTO".to_string(),
                query: "INSERT INTO".to_string(),
            },
        ),
        (
            "upd".to_string(),
            Snippet {
                shortcut: "upd".to_string(),
                description: "UPDATE".to_string(),
                query: "UPDATE".to_string(),
            },
        ),
        (
            "del".to_string(),
            Snippet {
                shortcut: "del".to_string(),
                description: "DELETE FROM".to_string(),
                query: "DELETE FROM".to_string(),
            },
        ),
        (
            "cre".to_string(),
            Snippet {
                shortcut: "cre".to_string(),
                description: "CREATE TABLE".to_string(),
                query: "CREATE TABLE".to_string(),
            },
        ),
        (
            "alt".to_string(),
            Snippet {
                shortcut: "alt".to_string(),
                description: "ALTER TABLE".to_string(),
                query: "ALTER TABLE".to_string(),
            },
        ),
        (
            "dro".to_string(),
            Snippet {
                shortcut: "dro".to_string(),
                description: "DROP TABLE".to_string(),
                query: "DROP TABLE".to_string(),
            },
        ),
        (
            "trun".to_string(),
            Snippet {
                shortcut: "trun".to_string(),
                description: "TRUNCATE TABLE".to_string(),
                query: "TRUNCATE TABLE".to_string(),
            },
        ),
        (
            "cq".to_string(),
            Snippet {
                shortcut: "cq".to_string(),
                description: "Current queries".to_string(),
                query: "SELECT pid, age(clock_timestamp(), query_start), usename, query, state
FROM pg_stat_activity
WHERE state != 'idle' AND query NOT ILIKE '%pg_stat_activity%'
ORDER BY query_start desc;"
                    .to_string(),
            },
        ),
        (
            "ds".to_string(),
            Snippet {
                shortcut: "ds".to_string(),
                description: "Databases sizes".to_string(),
                query: "SELECT datname, pg_size_pretty(pg_database_size(datname))
FROM pg_database
ORDER BY pg_database_size(datname) DESC;"
                    .to_string(),
            },
        ),
        (
            "ts".to_string(),
            Snippet {
                shortcut: "ts".to_string(),
                description: "Tables sizes".to_string(),
                query: r#"SELECT nspname || '.' || relname AS "relation",
pg_size_pretty(pg_total_relation_size(C.oid)) AS "total_size"
FROM pg_class C
LEFT JOIN pg_namespace N ON (N.oid = C.relnamespace)
WHERE nspname NOT IN ('pg_catalog', 'information_schema')
AND C.relkind <> 'i'
AND nspname !~ '^pg_toast'
ORDER BY pg_total_relation_size(C.oid) DESC;"#
                    .to_string(),
            },
        ),
        (
            "cl".to_string(),
            Snippet {
                shortcut: "cl".to_string(),
                description: "Current locks".to_string(),
                query:
                    r#"SELECT t.relname, l.locktype, page, virtualtransaction, pid, mode, granted 
FROM pg_locks l, pg_stat_all_tables t 
WHERE l.relation = t.relid ORDER BY relation asc;"#
                        .to_string(),
            },
        ),
    ])
}
