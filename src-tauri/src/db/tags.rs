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

pub async fn rename_tag(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    old_name: &str,
    new_name: &str,
) -> Result<(), sqlx::Error> {
    let normalized_old = normalize_tag(old_name)?;
    let normalized_new = normalize_tag(new_name)?;

    if normalized_old == normalized_new {
        return Ok(());
    }

    let old_exists = sqlx::query_scalar::<_, String>(
        "SELECT id FROM tags WHERE name = ?"
    )
        .bind(&normalized_old)
        .fetch_optional(&mut **tx)
        .await?;

    if old_exists.is_none() {
        return Err(sqlx::Error::InvalidArgument("tag not found".into()));
    }

    let new_exists = sqlx::query_scalar::<_, String>(
        "SELECT id FROM tags WHERE name = ?"
    )
        .bind(&normalized_new)
        .fetch_optional(&mut **tx)
        .await?;

    if new_exists.is_some() {
        return Err(sqlx::Error::InvalidArgument("a tag with this name already exists".into()));
    }

    sqlx::query("UPDATE tags SET name = ? WHERE name = ?")
        .bind(&normalized_new)
        .bind(&normalized_old)
        .execute(&mut **tx)
        .await?;

    Ok(())
}

pub async fn merge_tag(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    source_name: &str,
    target_name: &str,
) -> Result<(), sqlx::Error> {
    let normalized_source = normalize_tag(source_name)?;
    let normalized_target = normalize_tag(target_name)?;

    if normalized_source == normalized_target {
        return Ok(());
    }

    let source_id = sqlx::query_scalar::<_, String>(
        "SELECT id FROM tags WHERE name = ?"
    )
        .bind(&normalized_source)
        .fetch_optional(&mut **tx)
        .await?;

    let source_id = match source_id {
        Some(source_id) => source_id,
        None => return Err(sqlx::Error::InvalidArgument("source tag not found".into())),
    };

    let target_id = sqlx::query_scalar::<_, String>(
        "SELECT id FROM tags WHERE name = ?"
    )
        .bind(&normalized_target)
        .fetch_optional(&mut **tx)
        .await?;

    let target_id = match target_id {
        Some(target_id) => target_id,
        None => return Err(sqlx::Error::InvalidArgument("target tag not found".into())),
    };

    sqlx::query(
        "INSERT OR IGNORE INTO snippet_tags (snippet_id, tag_id) SELECT snippet_id, ? FROM snippet_tags WHERE tag_id = ?"
    )
        .bind(&target_id)
        .bind(&source_id)
        .execute(&mut **tx)
        .await?;

    sqlx::query("DELETE FROM tags WHERE id = ?")
        .bind(&source_id)
        .execute(&mut **tx)
        .await?;

    Ok(())
}
