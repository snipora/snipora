use crate::db::helper::{build_fts_query, parse_query};
use crate::db::tags::get_tags_for_snippet;

pub struct Snippet {
    pub id: String,
    pub label: String,
    pub snippet: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub last_used_at: Option<i64>,
}

pub struct SnippetWithTags {
    pub id: String,
    pub label: String,
    pub snippet: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub last_used_at: Option<i64>,
    pub tags: Vec<String>,
}

pub fn create_snippet(
    tx: &rusqlite::Transaction,
    label: String,
    snippet: String,
) -> rusqlite::Result<String> {
    let id = uuid::Uuid::now_v7().to_string();
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    tx.execute(
        r#"
INSERT INTO snippets (id, label, snippet, created_at, updated_at)
VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
        rusqlite::params![id, label, snippet, now, now],
    )?;

    Ok(id)
}

pub fn get_snippet_by_id(
    conn: &rusqlite::Connection,
    id: &str,
) -> rusqlite::Result<Option<Snippet>> {
    let mut stmt = conn.prepare(
        r#"
SELECT id, label, snippet, created_at, updated_at, last_used_at
FROM snippets
WHERE id = ?1
        "#,
    )?;

    let mut rows = stmt.query([id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Snippet {
            id: row.get(0)?,
            label: row.get(1)?,
            snippet: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            last_used_at: row.get(5)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn get_snippet_with_tags_by_id(
    conn: &rusqlite::Connection,
    id: &str,
) -> rusqlite::Result<Option<SnippetWithTags>> {
    let snippet = get_snippet_by_id(&conn, id)?;

    if let Some(snippet) = snippet {
        let tags = get_tags_for_snippet(conn, &snippet.id)?;

        Ok(Some(SnippetWithTags {
            id: snippet.id,
            label: snippet.label,
            snippet: snippet.snippet,
            created_at: snippet.created_at,
            updated_at: snippet.updated_at,
            last_used_at: snippet.last_used_at,
            tags: tags,
        }))
    } else {
        Ok(None)
    }
}

pub fn update_snippet(
    tx: &rusqlite::Transaction,
    id: &str,
    label: String,
    snippet: String,
) -> rusqlite::Result<()> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    tx.execute(
        r#"
UPDATE snippets
SET
    label = ?2,
    snippet = ?3,
    updated_at = ?4
WHERE id = ?1
        "#,
        rusqlite::params![id, label, snippet, now],
    )?;

    Ok(())
}

pub fn delete_snippet(
    tx: &rusqlite::Transaction,
    id: &str,
) -> rusqlite::Result<()> {
    tx.execute(
        r#"
DELETE FROM snippets
WHERE id = ?1
        "#,
        [id],
    )?;

    Ok(())
}

pub fn mark_used(
    tx: &rusqlite::Transaction,
    id: &str,
) -> rusqlite::Result<()> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    tx.execute(
        r#"
UPDATE snippets
SET last_used_at = ?2
WHERE id = ?1
        "#,
        rusqlite::params![id, now],
    )?;

    Ok(())
}

pub fn get_all_snippets_with_tags(
    conn: &rusqlite::Connection,
) -> rusqlite::Result<Vec<SnippetWithTags>> {
    let mut stmt = conn.prepare(
        r#"
SELECT s.id, s.label, s.snippet, s.created_at, s.updated_at, s.last_used_at, t.name
FROM snippets s
LEFT JOIN snippet_tags st
    ON s.id = st.snippet_id
LEFT JOIN tags t
    ON st.tag_id = t.id
ORDER BY s.updated_at DESC
        "#,
    )?;

    let mut map = std::collections::HashMap::<String, SnippetWithTags>::new();

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i64>(3)?,
            row.get::<_, i64>(4)?,
            row.get::<_, Option<i64>>(5)?,
            row.get::<_, Option<String>>(6)?,
        ))
    })?;

    for row in rows {
        let (id, label, snippet, created_at, updated_at, last_used_at, tag) = row?;

        let entry = map.entry(id.clone()).or_insert(SnippetWithTags {
            id,
            label,
            snippet,
            created_at,
            updated_at,
            last_used_at,
            tags: vec![],
        });

        if let Some(tag) = tag {
            entry.tags.push(tag);
        }
    }

    Ok(map.into_values().collect())
}

pub fn search_snippets(
    conn: &rusqlite::Connection,
    query: &str,
    limit: Option<i64>,
) -> rusqlite::Result<Vec<SnippetWithTags>> {
    let (terms, tags) = parse_query(&query);

    if terms.is_empty() && tags.is_empty() {
        return Err(rusqlite::Error::InvalidParameterName("query is empty".into()));
    }

    let fts_query = if terms.is_empty() {
        None
    } else {
        Some(build_fts_query(&terms))
    };

    let mut sql_str = String::from(r#"
SELECT s.id, s.label, s.snippet, s.created_at, s.updated_at, s.last_used_at, t.name
FROM snippets s
    "#);

    if fts_query.is_some() {
        sql_str.push_str(r#"
JOIN snippets_fts
    ON s.rowid = snippets_fts.rowid
        "#);
    }

    sql_str.push_str(r#"
LEFT JOIN snippet_tags st
    ON s.id = st.snippet_id
LEFT JOIN tags t
    ON st.tag_id = t.id
WHERE 1=1
    "#);

    if fts_query.is_some() {
        sql_str.push_str(r#"
AND snippets_fts MATCH ?1
        "#);
    }

    if !tags.is_empty() {
        let placeholders = (0..tags.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");

        sql_str.push_str(&format!(
            r#"
AND s.id IN (
    SELECT st.snippet_id
    FROM snippet_tags st
    JOIN tags t
        ON t.id = st.tag_id
    WHERE t.name in ({})
    GROUP BY st.snippet_id
    HAVING COUNT(DISTINCT t.name) = {}
)
            "#,
            placeholders,
            tags.len(),
        ));
    }

    if fts_query.is_some() {
        sql_str.push_str(r#"
ORDER BY bm25(snippets_fts)
        "#);
    } else {
        sql_str.push_str(r#"
ORDER BY s.last_used_at DESC NULLS LAST
        "#);
    }

    if limit.is_some() {
        sql_str.push_str(r#"
    LIMIT ?
        "#);
    }

    let mut stmt = conn.prepare(&sql_str)?;

    let mut params: Vec<&dyn rusqlite::ToSql> = vec![];

    if let Some(ref q) = fts_query {
        params.push(q);
    }

    for tag in &tags {
        params.push(tag);
    }

    if limit.is_some() {
        params.push(&limit);
    }

    let rows = stmt.query_map(&params[..], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i64>(3)?,
            row.get::<_, i64>(4)?,
            row.get::<_, Option<i64>>(5)?,
            row.get::<_, Option<String>>(6)?,
        ))
    })?;

    let mut map = std::collections::HashMap::<String, SnippetWithTags>::new();

    for row in rows {
        let (id, label, snippet, created_at, updated_at, last_used_at, tag) = row?;

        let entry = map.entry(id.clone()).or_insert(SnippetWithTags {
            id,
            label,
            snippet,
            created_at,
            updated_at,
            last_used_at,
            tags: vec![],
        });

        if let Some(tag) = tag {
            if !entry.tags.contains(&tag) {
                entry.tags.push(tag);
            }
        }
    }

    Ok(map.into_values().collect())
}
