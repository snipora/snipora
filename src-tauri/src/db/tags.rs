fn normalize_tag(input: &str) -> rusqlite::Result<String> {
    let tag = input.trim().to_lowercase();

    if tag.is_empty() {
        return Err(rusqlite::Error::InvalidParameterName("empty tag".into()));
    }

    if tag.len() > 32 {
        return Err(rusqlite::Error::InvalidParameterName("tag too long".into()));
    }

    if !tag
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        return Err(rusqlite::Error::InvalidParameterName("invalid characters in tag".into()));
    }

    Ok(tag)
}

pub fn get_all_tags(
    conn: &rusqlite::Connection,
) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare(r#"
SELECT name
FROM tags
ORDER BY name ASC
    "#)?;
    
    let rows = stmt.query_map([], |row| row.get(0))?;

    Ok(rows.filter_map(Result::ok).collect())
}

pub fn get_tags_for_snippet(
    conn: &rusqlite::Connection,
    snippet_id: &str,
) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare(r#"
SELECT t.name
FROM tags t
JOIN snippet_tags st
    ON t.id = st.tag_id
WHERE st.snippet_id = ?1
    "#)?;
    
    let rows = stmt.query_map([snippet_id], |row| row.get(0))?;

    Ok(rows.filter_map(Result::ok).collect())
}

pub fn get_or_create_tag(
    tx: &rusqlite::Transaction,
    name: &str,
) -> rusqlite::Result<String> {
    let normalized = normalize_tag(name)?;

    let mut stmt = tx.prepare(r#"
SELECT id
FROM tags
WHERE name = ?1
    "#)?;
    let mut rows = stmt.query(rusqlite::params![&normalized])?;

    if let Some(row) = rows.next()? {
        return Ok(row.get(0)?);
    }

    let id = uuid::Uuid::now_v7().to_string();

    tx.execute(
        r#"
INSERT INTO tags(id, name) VALUES (?1, ?2)
        "#,
        rusqlite::params![id, normalized],
    )?;

    Ok(id)
}

pub fn set_snippet_tags(
    tx: &rusqlite::Transaction,
    snippet_id: &str,
    tags: Vec<String>,
) -> rusqlite::Result<()> {
    let mut unique_tags = std::collections::HashSet::new();
    for tag in tags {
        let normalized = normalize_tag(&tag)?;
        unique_tags.insert(normalized);
    }

    tx.execute(
        r#"
DELETE FROM snippet_tags
WHERE snippet_id = ?1
        "#,
        rusqlite::params![snippet_id],
    )?;

    for tag in unique_tags {
        let tag_id = get_or_create_tag(&tx, &tag)?;

        tx.execute(
            r#"
INSERT OR IGNORE INTO snippet_tags(snippet_id, tag_id)
VALUES (?1, ?2)
            "#,
            rusqlite::params![snippet_id, tag_id],
        )?;
    }

    Ok(())
}
