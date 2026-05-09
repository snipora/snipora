use tauri::Emitter;

pub fn emit_data_changed(app: tauri::AppHandle) {
    log::debug!("emit:data_changed()");
    app.emit("data-changed", ())
        .expect("couldn't emit data-changed event");
}
