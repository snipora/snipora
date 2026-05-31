use sqlx::Row;

use crate::db::utils::{build_fts_query, parse_query};

#[derive(sqlx::FromRow)]
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

pub async fn create_snippet(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    label: String,
    snippet: String,
) -> Result<String, sqlx::Error> {
    let id = uuid::Uuid::now_v7().to_string();
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    sqlx::query(r#"
INSERT INTO snippets (id, label, snippet, created_at, updated_at)
VALUES (?, ?, ?, ?, ?)
        "#)
        .bind(&id)
        .bind(&label)
        .bind(&snippet)
        .bind(now)
        .bind(now)
        .execute(&mut **tx)
        .await?;

    Ok(id)
}

pub async fn get_snippet_by_id(
    pool: &sqlx::SqlitePool,
    id: &str,
) -> Result<Option<Snippet>, sqlx::Error> {
    sqlx::query_as::<_, Snippet>(r#"
SELECT id, label, snippet, created_at, updated_at, last_used_at
FROM snippets
WHERE id = ?
        "#)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn get_snippet_with_tags_by_id(
    pool: &sqlx::SqlitePool,
    id: &str,
) -> Result<Option<SnippetWithTags>, sqlx::Error> {
    let snippet = get_snippet_by_id(pool, id).await?;

    if let Some(snippet) = snippet {
        let tags = crate::db::tags::get_tags_for_snippet(pool, &snippet.id)
            .await?;

        Ok(Some(SnippetWithTags {
            id: snippet.id,
            label: snippet.label,
            snippet: snippet.snippet,
            created_at: snippet.created_at,
            updated_at: snippet.updated_at,
            last_used_at: snippet.last_used_at,
            tags,
        }))
    } else {
        Ok(None)
    }
}

pub async fn update_snippet(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    id: &str,
    label: String,
    snippet: String,
) -> Result<(), sqlx::Error> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    sqlx::query(r#"
UPDATE snippets
SET
    label = ?,
    snippet = ?,
    updated_at = ?
WHERE id = ?
        "#)
        .bind(&label)
        .bind(&snippet)
        .bind(now)
        .bind(id)
        .execute(&mut **tx)
        .await?;

    Ok(())
}

pub async fn delete_snippet(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(r#"
DELETE FROM snippets
WHERE id = ?
        "#)
        .bind(id)
        .execute(&mut **tx)
        .await?;

    Ok(())
}

pub async fn mark_used(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    id: &str,
) -> Result<(), sqlx::Error> {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    sqlx::query(r#"
UPDATE snippets
SET last_used_at = ?
WHERE id = ?
        "#)
        .bind(now)
        .bind(id)
        .execute(&mut **tx)
        .await?;

    Ok(())
}

pub async fn get_all_snippets_with_tags(
    pool: &sqlx::SqlitePool,
) -> Result<Vec<SnippetWithTags>, sqlx::Error> {
    let rows = sqlx::query(r#"
SELECT s.id, s.label, s.snippet, s.created_at, s.updated_at, s.last_used_at, t.name
FROM snippets s
LEFT JOIN snippet_tags st
    ON s.id = st.snippet_id
LEFT JOIN tags t
    ON st.tag_id = t.id
ORDER BY s.updated_at DESC
        "#)
        .fetch_all(pool)
        .await?;

    let mut map = std::collections::HashMap::<String, SnippetWithTags>::new();

    for row in &rows {
        let id: String = row.try_get(0)?;
        let label: String = row.try_get(1)?;
        let snippet: String = row.try_get(2)?;
        let created_at: i64 = row.try_get(3)?;
        let updated_at: i64 = row.try_get(4)?;
        let last_used_at: Option<i64> = row.try_get(5)?;
        let tag: Option<String> = row.try_get(6)?;

        let entry = map
            .entry(id.clone())
            .or_insert_with(|| SnippetWithTags {
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

pub async fn search_snippets(
    pool: &sqlx::SqlitePool,
    query: &str,
    limit: Option<i64>,
) -> Result<Vec<SnippetWithTags>, sqlx::Error> {
    let (terms, tags) = parse_query(query);

    if terms.is_empty() && tags.is_empty() {
        return Err(sqlx::Error::InvalidArgument("query is empty".into()));
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
AND snippets_fts MATCH ?
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

    let mut q = sqlx::query(&sql_str);

    if let Some(ref q_val) = fts_query {
        q = q.bind(q_val);
    }

    for tag in &tags {
        q = q.bind(tag);
    }

    if let Some(limit_val) = limit {
        q = q.bind(limit_val);
    }

    let rows = q.fetch_all(pool).await?;

    let mut map = std::collections::HashMap::<String, SnippetWithTags>::new();

    for row in &rows {
        let id: String = row.try_get(0)?;
        let label: String = row.try_get(1)?;
        let snippet: String = row.try_get(2)?;
        let created_at: i64 = row.try_get(3)?;
        let updated_at: i64 = row.try_get(4)?;
        let last_used_at: Option<i64> = row.try_get(5)?;
        let tag: Option<String> = row.try_get(6)?;

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
