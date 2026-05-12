use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use arrow_array::{Array, Int64Array, RecordBatch, StringArray};
use futures::TryStreamExt;
use lancedb::Connection;
use lancedb::arrow::arrow_schema::{DataType, Field, Schema};
use lancedb::query::{ExecutableQuery, QueryBase};

use serde::{Deserialize, Deserializer, Serialize};

const OBJECTS_TABLE: &str = "objects";
const CONNECTIONS_TABLE: &str = "connections";

const MARKDOWN_NOTE_OBJECT_TYPE: &str = "markdown_note";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectRecord {
    id: String,
    object_type: String,
    title: String,
    content_text: String,
    metadata_json: String,
    source_uri: Option<String>,
    created_at_ms: i64,
    updated_at_ms: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectUpdate {
    id: String,
    object_type: Option<String>,
    title: Option<String>,
    content_text: Option<String>,
    metadata_json: Option<String>,
    #[serde(default, deserialize_with = "deserialize_nullable_update")]
    source_uri: Option<Option<String>>,
    created_at_ms: Option<i64>,
    updated_at_ms: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectConnectionRecord {
    id: String,
    source_object_id: String,
    target_object_id: String,
    content_text: String,
    created_at_ms: i64,
    updated_at_ms: i64,
}

pub async fn connect() -> lancedb::Result<Connection> {
    let db = lancedb::connect("../../data")
        .read_consistency_interval(Duration::from_secs(0))
        .execute()
        .await?;

    ensure_objects_table(&db).await?;
    ensure_connections_table(&db).await?;
    Ok(db)
}

#[tauri::command]
pub async fn search_objects(
    db: tauri::State<'_, Connection>,
    query: Option<String>,
) -> Result<Vec<ObjectRecord>, String> {
    let table = db
        .open_table(OBJECTS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let mut q = table.query();

    if let Some(query) = query {
        let trimmed = query.trim();
        if !trimmed.is_empty() {
            let needle = sql_escape(&trimmed.to_lowercase());
            let filter =
                format!("LOWER(title) LIKE '%{needle}%' OR LOWER(content_text) LIKE '%{needle}%'");
            q = q.only_if(filter);
        }
    }

    let batches: Vec<RecordBatch> = q
        .execute()
        .await
        .map_err(|err| err.to_string())?
        .try_collect()
        .await
        .map_err(|err| err.to_string())?;

    let mut objects = Vec::new();
    for batch in batches {
        objects.extend(objects_from_batch(&batch)?);
    }

    objects.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    Ok(objects)
}

#[tauri::command]
pub async fn create_markdown_object(
    db: tauri::State<'_, Connection>,
    content_text: String,
) -> Result<ObjectRecord, String> {
    let now_ms = now_millis()?;
    let title = markdown_title_or_default(&content_text);
    let object = ObjectRecord {
        id: now_nanos()?.to_string(),
        object_type: MARKDOWN_NOTE_OBJECT_TYPE.to_string(),
        title,
        content_text,
        metadata_json: "{}".to_string(),
        source_uri: None,
        created_at_ms: now_ms,
        updated_at_ms: now_ms,
    };

    let table = db
        .open_table(OBJECTS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let schema = object_schema();
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(vec![object.id.clone()])),
            Arc::new(StringArray::from(vec![object.object_type.clone()])),
            Arc::new(StringArray::from(vec![object.title.clone()])),
            Arc::new(StringArray::from(vec![object.content_text.clone()])),
            Arc::new(StringArray::from(vec![object.metadata_json.clone()])),
            Arc::new(StringArray::from(vec![object.source_uri.as_deref()])),
            Arc::new(Int64Array::from(vec![object.created_at_ms])),
            Arc::new(Int64Array::from(vec![object.updated_at_ms])),
        ],
    )
    .map_err(|err| err.to_string())?;

    table
        .add(batch)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    Ok(object)
}

#[tauri::command]
pub async fn load_object(
    db: tauri::State<'_, Connection>,
    id: String,
) -> Result<ObjectRecord, String> {
    load_object_by_id(db.inner(), &id).await
}

#[tauri::command]
pub async fn update_object(
    db: tauri::State<'_, Connection>,
    update: ObjectUpdate,
) -> Result<ObjectRecord, String> {
    if update.id.trim().is_empty() {
        return Err("Object id is required".to_string());
    }

    let existing = load_object_by_id(db.inner(), &update.id).await?;
    let now_ms = now_millis()?;
    let object_type = update.object_type.unwrap_or(existing.object_type);
    let content_text = update.content_text.unwrap_or(existing.content_text);
    let requested_title = update.title.unwrap_or(existing.title);
    let title = title_for_object(&object_type, &requested_title, &content_text);

    let object = ObjectRecord {
        id: existing.id,
        object_type,
        title,
        content_text,
        metadata_json: update.metadata_json.unwrap_or(existing.metadata_json),
        source_uri: update.source_uri.unwrap_or(existing.source_uri),
        created_at_ms: update.created_at_ms.unwrap_or(existing.created_at_ms),
        updated_at_ms: update.updated_at_ms.unwrap_or(now_ms),
    };

    let table = db
        .open_table(OBJECTS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let escaped_id = sql_escape(&object.id);
    let result = table
        .update()
        .only_if(format!("id = '{escaped_id}'"))
        .column("object_type", sql_string_literal(&object.object_type))
        .column("title", sql_string_literal(&object.title))
        .column("content_text", sql_string_literal(&object.content_text))
        .column("metadata_json", sql_string_literal(&object.metadata_json))
        .column(
            "source_uri",
            sql_nullable_string_literal(object.source_uri.as_deref()),
        )
        .column("created_at_ms", object.created_at_ms.to_string())
        .column("updated_at_ms", object.updated_at_ms.to_string())
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    if result.rows_updated == 0 {
        return Err(format!("Object {} not found", object.id));
    }

    Ok(object)
}

#[tauri::command]
pub async fn save_object_content(
    db: tauri::State<'_, Connection>,
    id: String,
    content_text: String,
) -> Result<(), String> {
    update_object(
        db,
        ObjectUpdate {
            id,
            object_type: None,
            title: None,
            content_text: Some(content_text),
            metadata_json: None,
            source_uri: None,
            created_at_ms: None,
            updated_at_ms: None,
        },
    )
    .await
    .map(|_| ())
}

#[tauri::command]
pub async fn delete_object(db: tauri::State<'_, Connection>, id: String) -> Result<(), String> {
    if id.trim().is_empty() {
        return Err("Object id is required".to_string());
    }

    let escaped_id = sql_escape(&id);
    let connection_filter =
        format!("source_object_id = '{escaped_id}' OR target_object_id = '{escaped_id}'");

    let connections_table = db
        .open_table(CONNECTIONS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    connections_table
        .delete(&connection_filter)
        .await
        .map_err(|err| err.to_string())?;

    let objects_table = db
        .open_table(OBJECTS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let result = objects_table
        .delete(&format!("id = '{escaped_id}'"))
        .await
        .map_err(|err| err.to_string())?;

    if result.num_deleted_rows == 0 {
        return Err(format!("Object {id} not found"));
    }

    Ok(())
}

#[tauri::command]
pub async fn list_all_connections(
    db: tauri::State<'_, Connection>,
) -> Result<Vec<ObjectConnectionRecord>, String> {
    let table = db
        .open_table(CONNECTIONS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let batches: Vec<RecordBatch> = table
        .query()
        .execute()
        .await
        .map_err(|err| err.to_string())?
        .try_collect()
        .await
        .map_err(|err| err.to_string())?;

    let mut connections = Vec::new();
    for batch in batches {
        connections.extend(connections_from_batch(&batch)?);
    }

    connections.sort_by(|a, b| b.updated_at_ms.cmp(&a.updated_at_ms));
    Ok(connections)
}

#[tauri::command]
pub async fn list_object_connections(
    db: tauri::State<'_, Connection>,
    object_id: String,
) -> Result<Vec<ObjectConnectionRecord>, String> {
    if object_id.trim().is_empty() {
        return Err("Object id is required".to_string());
    }

    let table = db
        .open_table(CONNECTIONS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let escaped_id = sql_escape(&object_id);
    let batches: Vec<RecordBatch> = table
        .query()
        .only_if(format!(
            "source_object_id = '{escaped_id}' OR target_object_id = '{escaped_id}'"
        ))
        .execute()
        .await
        .map_err(|err| err.to_string())?
        .try_collect()
        .await
        .map_err(|err| err.to_string())?;

    let mut connections = Vec::new();
    for batch in batches {
        connections.extend(connections_from_batch(&batch)?);
    }

    connections.sort_by(|a, b| b.updated_at_ms.cmp(&a.updated_at_ms));
    Ok(connections)
}

#[tauri::command]
pub async fn create_object_connection(
    db: tauri::State<'_, Connection>,
    source_object_id: String,
    target_object_id: String,
    content_text: String,
) -> Result<ObjectConnectionRecord, String> {
    if source_object_id.trim().is_empty() || target_object_id.trim().is_empty() {
        return Err("Source and target object ids are required".to_string());
    }

    if source_object_id == target_object_id {
        return Err("A connection needs two different objects".to_string());
    }

    load_object_by_id(db.inner(), &source_object_id).await?;
    load_object_by_id(db.inner(), &target_object_id).await?;

    let now_ms = now_millis()?;
    let connection = ObjectConnectionRecord {
        id: now_nanos()?.to_string(),
        source_object_id,
        target_object_id,
        content_text,
        created_at_ms: now_ms,
        updated_at_ms: now_ms,
    };

    let table = db
        .open_table(CONNECTIONS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let schema = connection_schema();
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(vec![connection.id.clone()])),
            Arc::new(StringArray::from(vec![connection.source_object_id.clone()])),
            Arc::new(StringArray::from(vec![connection.target_object_id.clone()])),
            Arc::new(StringArray::from(vec![connection.content_text.clone()])),
            Arc::new(Int64Array::from(vec![connection.created_at_ms])),
            Arc::new(Int64Array::from(vec![connection.updated_at_ms])),
        ],
    )
    .map_err(|err| err.to_string())?;

    table
        .add(batch)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    Ok(connection)
}

#[tauri::command]
pub async fn update_object_connection_content(
    db: tauri::State<'_, Connection>,
    id: String,
    content_text: String,
) -> Result<ObjectConnectionRecord, String> {
    if id.trim().is_empty() {
        return Err("Connection id is required".to_string());
    }

    let existing = load_connection_by_id(db.inner(), &id).await?;
    let now_ms = now_millis()?;
    let connection = ObjectConnectionRecord {
        id: existing.id,
        source_object_id: existing.source_object_id,
        target_object_id: existing.target_object_id,
        content_text,
        created_at_ms: existing.created_at_ms,
        updated_at_ms: now_ms,
    };

    let table = db
        .open_table(CONNECTIONS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let escaped_id = sql_escape(&connection.id);
    let result = table
        .update()
        .only_if(format!("id = '{escaped_id}'"))
        .column("content_text", sql_string_literal(&connection.content_text))
        .column("updated_at_ms", connection.updated_at_ms.to_string())
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    if result.rows_updated == 0 {
        return Err(format!("Connection {} not found", connection.id));
    }

    Ok(connection)
}

#[tauri::command]
pub async fn delete_object_connection(
    db: tauri::State<'_, Connection>,
    id: String,
) -> Result<(), String> {
    if id.trim().is_empty() {
        return Err("Connection id is required".to_string());
    }

    let table = db
        .open_table(CONNECTIONS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let escaped_id = sql_escape(&id);
    let result = table
        .delete(&format!("id = '{escaped_id}'"))
        .await
        .map_err(|err| err.to_string())?;

    if result.num_deleted_rows == 0 {
        return Err(format!("Connection {id} not found"));
    }

    Ok(())
}

async fn load_object_by_id(db: &Connection, id: &str) -> Result<ObjectRecord, String> {
    let table = db
        .open_table(OBJECTS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let escaped = sql_escape(id);
    let batches: Vec<RecordBatch> = table
        .query()
        .only_if(format!("id = '{escaped}'"))
        .limit(1)
        .execute()
        .await
        .map_err(|err| err.to_string())?
        .try_collect()
        .await
        .map_err(|err| err.to_string())?;

    for batch in batches {
        if let Some(object) = objects_from_batch(&batch)?.pop() {
            return Ok(object);
        }
    }

    Err(format!("Object {id} not found"))
}

async fn load_connection_by_id(
    db: &Connection,
    id: &str,
) -> Result<ObjectConnectionRecord, String> {
    let table = db
        .open_table(CONNECTIONS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let escaped = sql_escape(id);
    let batches: Vec<RecordBatch> = table
        .query()
        .only_if(format!("id = '{escaped}'"))
        .limit(1)
        .execute()
        .await
        .map_err(|err| err.to_string())?
        .try_collect()
        .await
        .map_err(|err| err.to_string())?;

    for batch in batches {
        if let Some(connection) = connections_from_batch(&batch)?.pop() {
            return Ok(connection);
        }
    }

    Err(format!("Connection {id} not found"))
}

async fn ensure_objects_table(db: &Connection) -> lancedb::Result<()> {
    let exists = db
        .table_names()
        .execute()
        .await?
        .iter()
        .any(|n| n == OBJECTS_TABLE);

    if !exists {
        db.create_empty_table(OBJECTS_TABLE, object_schema())
            .execute()
            .await?;
    }

    Ok(())
}

async fn ensure_connections_table(db: &Connection) -> lancedb::Result<()> {
    let exists = db
        .table_names()
        .execute()
        .await?
        .iter()
        .any(|n| n == CONNECTIONS_TABLE);

    if !exists {
        db.create_empty_table(CONNECTIONS_TABLE, connection_schema())
            .execute()
            .await?;
    }

    Ok(())
}

fn object_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("object_type", DataType::Utf8, false),
        Field::new("title", DataType::Utf8, false),
        Field::new("content_text", DataType::Utf8, false),
        Field::new("metadata_json", DataType::Utf8, false),
        Field::new("source_uri", DataType::Utf8, true),
        Field::new("created_at_ms", DataType::Int64, false),
        Field::new("updated_at_ms", DataType::Int64, false),
    ]))
}

fn connection_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("source_object_id", DataType::Utf8, false),
        Field::new("target_object_id", DataType::Utf8, false),
        Field::new("content_text", DataType::Utf8, false),
        Field::new("created_at_ms", DataType::Int64, false),
        Field::new("updated_at_ms", DataType::Int64, false),
    ]))
}

fn objects_from_batch(batch: &RecordBatch) -> Result<Vec<ObjectRecord>, String> {
    let ids = column_as_strings(batch, "id")?;
    let object_types = column_as_strings(batch, "object_type")?;
    let titles = column_as_strings(batch, "title")?;
    let content_texts = column_as_strings(batch, "content_text")?;
    let metadata_jsons = column_as_strings(batch, "metadata_json")?;
    let source_uris = column_as_strings(batch, "source_uri")?;
    let created_at_values = column_as_int64(batch, "created_at_ms")?;
    let updated_at_values = column_as_int64(batch, "updated_at_ms")?;

    let mut objects = Vec::with_capacity(batch.num_rows());
    for i in 0..batch.num_rows() {
        objects.push(ObjectRecord {
            id: ids.value(i).to_string(),
            object_type: object_types.value(i).to_string(),
            title: titles.value(i).to_string(),
            content_text: content_texts.value(i).to_string(),
            metadata_json: metadata_jsons.value(i).to_string(),
            source_uri: optional_string_value(source_uris, i),
            created_at_ms: created_at_values.value(i),
            updated_at_ms: updated_at_values.value(i),
        });
    }
    Ok(objects)
}

fn connections_from_batch(batch: &RecordBatch) -> Result<Vec<ObjectConnectionRecord>, String> {
    let ids = column_as_strings(batch, "id")?;
    let source_object_ids = column_as_strings(batch, "source_object_id")?;
    let target_object_ids = column_as_strings(batch, "target_object_id")?;
    let content_texts = column_as_strings(batch, "content_text")?;
    let created_at_values = column_as_int64(batch, "created_at_ms")?;
    let updated_at_values = column_as_int64(batch, "updated_at_ms")?;

    let mut connections = Vec::with_capacity(batch.num_rows());
    for i in 0..batch.num_rows() {
        connections.push(ObjectConnectionRecord {
            id: ids.value(i).to_string(),
            source_object_id: source_object_ids.value(i).to_string(),
            target_object_id: target_object_ids.value(i).to_string(),
            content_text: content_texts.value(i).to_string(),
            created_at_ms: created_at_values.value(i),
            updated_at_ms: updated_at_values.value(i),
        });
    }
    Ok(connections)
}

fn column_as_strings<'a>(batch: &'a RecordBatch, name: &str) -> Result<&'a StringArray, String> {
    batch
        .column_by_name(name)
        .ok_or_else(|| format!("Missing column: {name}"))?
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| format!("Column {name} has unexpected type"))
}

fn column_as_int64<'a>(batch: &'a RecordBatch, name: &str) -> Result<&'a Int64Array, String> {
    batch
        .column_by_name(name)
        .ok_or_else(|| format!("Missing column: {name}"))?
        .as_any()
        .downcast_ref::<Int64Array>()
        .ok_or_else(|| format!("Column {name} has unexpected type"))
}

fn optional_string_value(values: &StringArray, index: usize) -> Option<String> {
    if values.is_null(index) {
        None
    } else {
        Some(values.value(index).to_string())
    }
}

fn title_for_object(object_type: &str, requested_title: &str, content_text: &str) -> String {
    if object_type == MARKDOWN_NOTE_OBJECT_TYPE {
        markdown_title_or_default(content_text)
    } else {
        normalize_title(requested_title)
    }
}

fn normalize_title(title: &str) -> String {
    let title = title.trim();
    if title.is_empty() {
        "Untitled".to_string()
    } else {
        title.to_string()
    }
}

fn markdown_title_or_default(content_text: &str) -> String {
    markdown_title(content_text).unwrap_or_else(|| "Untitled".to_string())
}

fn markdown_title(content_text: &str) -> Option<String> {
    content_text.lines().find_map(markdown_heading_title)
}

fn markdown_heading_title(line: &str) -> Option<String> {
    let line = line.trim_start();
    let rest = line.strip_prefix('#')?;

    if rest.starts_with('#') {
        return None;
    }

    match rest.chars().next() {
        Some(ch) if ch.is_whitespace() => {}
        _ => return None,
    }

    let title = strip_closing_heading_marker(rest.trim());
    let title = title.trim();

    if title.is_empty() {
        None
    } else {
        Some(title.to_string())
    }
}

fn strip_closing_heading_marker(title: &str) -> &str {
    let trimmed = title.trim_end();
    let without_hashes = trimmed.trim_end_matches('#');

    if without_hashes.len() != trimmed.len()
        && without_hashes
            .chars()
            .last()
            .is_some_and(char::is_whitespace)
    {
        without_hashes.trim_end()
    } else {
        trimmed
    }
}

fn sql_string_literal(s: &str) -> String {
    format!("'{}'", sql_escape(s))
}

fn sql_nullable_string_literal(s: Option<&str>) -> String {
    match s {
        Some(value) => sql_string_literal(value),
        None => "NULL".to_string(),
    }
}

fn sql_escape(s: &str) -> String {
    s.replace('\'', "''")
}

fn deserialize_nullable_update<'de, D>(deserializer: D) -> Result<Option<Option<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer).map(Some)
}

fn now_millis() -> Result<i64, String> {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())?
        .as_millis();

    i64::try_from(millis).map_err(|err| err.to_string())
}

fn now_nanos() -> Result<u128, String> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())?
        .as_nanos())
}

#[cfg(test)]
mod tests {
    use super::{ObjectUpdate, markdown_title, markdown_title_or_default};

    #[test]
    fn extracts_the_first_h1_title() {
        let markdown = "intro\n## Section\n# First Title\n# Second Title";

        assert_eq!(markdown_title(markdown).as_deref(), Some("First Title"));
    }

    #[test]
    fn strips_closing_heading_markers() {
        assert_eq!(
            markdown_title("# First Title ###").as_deref(),
            Some("First Title")
        );
    }

    #[test]
    fn preserves_hashes_that_are_part_of_the_title() {
        assert_eq!(markdown_title("# C#").as_deref(), Some("C#"));
    }

    #[test]
    fn falls_back_when_no_h1_exists() {
        assert_eq!(markdown_title_or_default("## Section"), "Untitled");
    }

    #[test]
    fn distinguishes_absent_and_null_source_uri_updates() {
        let absent_source_uri: ObjectUpdate = serde_json::from_str(r#"{"id":"1"}"#).unwrap();
        let null_source_uri: ObjectUpdate =
            serde_json::from_str(r#"{"id":"1","sourceUri":null}"#).unwrap();

        assert_eq!(absent_source_uri.source_uri, None);
        assert_eq!(null_source_uri.source_uri, Some(None));
    }
}
