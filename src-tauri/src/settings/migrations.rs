pub fn migrate(raw: serde_json::Value) -> Result<serde_json::Value, String> {
    #[allow(unused_variables)]  // todo: remove with first migration
    let version = raw
        .get("version")
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .expect("missing or invalid version");

    #[allow(unused_mut)]  // todo: remove with first migration
    let mut result = raw;

    // if version < 2 {
    //     result = migrate_to_v2(result);
    // }

    Ok(result)
}
