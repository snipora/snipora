use enigo::Keyboard;

use crate::settings::internal::SnippetUsageBehavior;

#[tauri::command]
pub async fn use_snippet(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    state_settings: tauri::State<'_, std::sync::Mutex<crate::settings::internal::LocalSettings>>,
    state_clipboard: tauri::State<'_, std::sync::Mutex<arboard::Clipboard>>,
    state_enigo: tauri::State<'_, std::sync::Mutex<enigo::Enigo>>,
    snippet_id: String,
) -> Result<(), String> {
    log::debug!("cmd:insert_snippet({:?})", snippet_id);

    let snippet = crate::db::snippets::get_snippet_by_id(&pool, &snippet_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("snippet not found")?;

    if snippet.snippet.trim().is_empty() {
        return Err("snippet is empty".into());
    }

    let behavior = {
        let settings = state_settings.lock()
            .expect("failed to lock settings");
        settings.general.snippet_usage_behavior.clone()
    };

    match behavior {
        SnippetUsageBehavior::CopyToClipboard => {
            let mut clipboard = state_clipboard.lock()
                .expect("failed to get clipboard");
            clipboard.set_text(snippet.snippet)
                .map_err(|e| e.to_string())?;
        }
        SnippetUsageBehavior::NaturalTyping => {
            let mut enigo = state_enigo.lock()
                .expect("failed to get enigo instance");
            enigo.text(&snippet.snippet)
                .map_err(|e| e.to_string())?;
        }
        SnippetUsageBehavior::SimulatePaste => {
            return Err("simulate-paste is not implemented yet".into());
        }
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| e.to_string())?;

    crate::db::snippets::mark_used(&mut tx, &snippet_id)
        .await
        .map_err(|e| e.to_string())?;

    tx.commit()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
