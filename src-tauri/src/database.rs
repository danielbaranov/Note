use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use arrow_array::{Array, Int64Array, RecordBatch, StringArray};
use futures::TryStreamExt;
use lancedb::Connection;
use lancedb::arrow::arrow_schema::{DataType, Field, Schema};
use lancedb::query::{ExecutableQuery, QueryBase};

use serde::Serialize;

const OBJECTS_TABLE: &str = "objects";

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

pub async fn connect() -> lancedb::Result<Connection> {
    let db = lancedb::connect("../../data")
        .read_consistency_interval(Duration::from_secs(0))
        .execute()
        .await?;

    ensure_objects_table(&db).await?;
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
    title: String,
) -> Result<ObjectRecord, String> {
    let now_ms = now_millis()?;
    let object = ObjectRecord {
        id: now_nanos()?.to_string(),
        object_type: MARKDOWN_NOTE_OBJECT_TYPE.to_string(),
        title: normalize_markdown_title(&title),
        content_text: String::new(),
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
    let table = db
        .open_table(OBJECTS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let escaped = sql_escape(&id);
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

#[tauri::command]
pub async fn save_object_content(
    db: tauri::State<'_, Connection>,
    id: String,
    content_text: String,
) -> Result<(), String> {
    let table = db
        .open_table(OBJECTS_TABLE)
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    let escaped_id = sql_escape(&id);
    let escaped_content_text = sql_escape(&content_text);
    let updated_at_ms = now_millis()?;

    let result = table
        .update()
        .only_if(format!("id = '{escaped_id}'"))
        .column("content_text", format!("'{escaped_content_text}'"))
        .column("updated_at_ms", updated_at_ms.to_string())
        .execute()
        .await
        .map_err(|err| err.to_string())?;

    if result.rows_updated == 0 {
        return Err(format!("Object {id} not found"));
    }

    Ok(())
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

fn normalize_markdown_title(title: &str) -> String {
    let title = title.trim();
    let title = if title.is_empty() { "Untitled" } else { title };

    if title.to_lowercase().ends_with(".md") {
        title.to_string()
    } else {
        format!("{title}.md")
    }
}

fn sql_escape(s: &str) -> String {
    s.replace('\'', "''")
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
