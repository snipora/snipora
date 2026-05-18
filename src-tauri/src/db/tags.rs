fn normalize_tag(input: &str) -> Result<String, sqlx::Error> {
    let tag = input.trim().to_lowercase();

    if tag.is_empty() {
        return Err(sqlx::Error::InvalidArgument("empty tag".into()));
    }

    if tag.len() > 32 {
        return Err(sqlx::Error::InvalidArgument("tag too long".into()));
    }

    if !tag
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        return Err(sqlx::Error::InvalidArgument("invalid characters in tag".into()));
    }

    Ok(tag)
}

pub async fn get_tags_for_snippet(
    pool: &sqlx::SqlitePool,
    snippet_id: &str,
) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query_scalar::<_, String>(r#"
SELECT t.name
FROM tags t
JOIN snippet_tags st
    ON t.id = st.tag_id
WHERE st.snippet_id = ?
        "#)
        .bind(snippet_id)
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

pub async fn get_or_create_tag(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    name: &str,
) -> Result<String, sqlx::Error> {
    let normalized = normalize_tag(name)?;

    let existing = sqlx::query_scalar::<_, String>(r#"
SELECT id
FROM tags
WHERE name = ?
        "#)
        .bind(&normalized)
        .fetch_optional(&mut **tx)
        .await?;

    if let Some(id) = existing {
        return Ok(id);
    }

    let id = uuid::Uuid::now_v7().to_string();

    sqlx::query(r#"
INSERT INTO tags(id, name) VALUES (?, ?)
        "#)
        .bind(&id)
        .bind(&normalized)
        .execute(&mut **tx)
        .await?;

    Ok(id)
}

pub async fn set_snippet_tags(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    snippet_id: &str,
    tags: Vec<String>,
) -> Result<(), sqlx::Error> {
    let mut unique_tags = std::collections::HashSet::new();
    for tag in tags {
        let normalized = normalize_tag(&tag)?;
        unique_tags.insert(normalized);
    }

    sqlx::query(r#"
DELETE FROM snippet_tags
WHERE snippet_id = ?
        "#)
        .bind(snippet_id)
        .execute(&mut **tx)
        .await?;

    for tag in unique_tags {
        let tag_id = get_or_create_tag(tx, &tag)
            .await?;

        sqlx::query(r#"
INSERT OR IGNORE INTO snippet_tags(snippet_id, tag_id)
VALUES (?, ?)
        "#)
        .bind(snippet_id)
        .bind(&tag_id)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}

pub async fn cleanup_unused_tags(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
) -> Result<(), sqlx::Error> {
    sqlx::query(r#"
DELETE FROM tags
WHERE id NOT IN (
    SELECT DISTINCT tag_id FROM snippet_tags
)
        "#)
        .execute(&mut **tx)
        .await?;

    Ok(())
}

pub async fn delete_tag(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    tag_name: &str,
) -> Result<(), sqlx::Error> {
    let normalized = normalize_tag(tag_name)?;

    sqlx::query(r#"
DELETE FROM snippet_tags
WHERE tag_id IN (
    SELECT id FROM tags WHERE name = ?
)
        "#)
        .bind(&normalized)
        .execute(&mut **tx)
        .await?;

    sqlx::query(r#"
DELETE FROM tags WHERE name = ?
        "#)
        .bind(&normalized)
        .execute(&mut **tx)
        .await?;

    Ok(())
}
