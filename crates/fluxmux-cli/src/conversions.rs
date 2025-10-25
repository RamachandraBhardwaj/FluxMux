/// -------- NDJSON Import/Export --------
pub fn import_ndjson(path: &str) -> Result<Value, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let lines: Vec<Value> = data
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l))
        .collect::<Result<_, _>>()?;
    Ok(Value::Array(lines))
}

pub fn export_ndjson(value: &Value, path: &str) -> Result<(), Box<dyn Error>> {
    let arr = value.as_array().ok_or("Expected array for NDJSON export")?;
    let mut out = String::new();
    for v in arr {
        out.push_str(&serde_json::to_string(v)?);
        out.push('\n');
    }
    fs::write(path, out)?;
    Ok(())
}

use serde_json::Value;
use std::error::Error;
use std::fs;

/// Supported formats
#[derive(Debug, Clone)]
pub enum Format {
    Json,
    Csv,
    Yaml,
    Toml,
    Parquet,
    Avro,
    MsgPack,
    Cbor,
    Ndjson,
}

impl Format {
    pub fn from_ext(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "json" => Some(Self::Json),
            "csv" => Some(Self::Csv),
            "yaml" | "yml" => Some(Self::Yaml),
            "toml" => Some(Self::Toml),
            "parquet" => Some(Self::Parquet),
            "avro" => Some(Self::Avro),
            "msgpack" => Some(Self::MsgPack),
            "cbor" => Some(Self::Cbor),
            "ndjson" => Some(Self::Ndjson),
            _ => None,
        }
    }
}



// -------- Parquet Import/Export --------
pub fn import_parquet(path: &str) -> Result<Value, Box<dyn Error>> {
    use parquet::file::reader::SerializedFileReader;
    use parquet::file::reader::FileReader;
    use std::fs::File;
    let file = File::open(path)?;
    let reader = SerializedFileReader::new(file)?;
    let mut rows = Vec::new();
    for row_result in reader.get_row_iter(None)? {
        let row = row_result?;
        let mut obj = serde_json::Map::new();
        for (name, field) in row.get_column_iter() {
            obj.insert(name.clone(), Value::String(field.to_string()));
        }
        rows.push(Value::Object(obj));
    }
    Ok(Value::Array(rows))
}

pub fn export_parquet(value: &Value, path: &str) -> Result<(), Box<dyn Error>> {
    // Parquet export requires schema definition. This is a simplified example for arrays of objects.
    use parquet::file::properties::WriterProperties;
    use parquet::arrow::ArrowWriter;
    use arrow::array::StringArray;
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use std::sync::Arc;
    use std::fs::File;
    let arr = value.as_array().ok_or("Expected array for Parquet export")?;
    if arr.is_empty() {
        return Err("No rows found for Parquet export".into());
    }
    let obj = arr[0].as_object().ok_or("Expected object rows in array")?;
    let fields: Vec<Field> = obj.keys().map(|k| Field::new(k, DataType::Utf8, true)).collect();
    let schema = Arc::new(Schema::new(fields.clone()));
    let mut columns: Vec<Vec<String>> = vec![Vec::new(); fields.len()];
    for row in arr {
        let obj = row.as_object().ok_or("Expected object row")?;
        for (i, k) in obj.keys().enumerate() {
            columns[i].push(obj.get(k).map(|v| v.to_string()).unwrap_or_default());
        }
    }
    let arrays: Vec<Arc<dyn arrow::array::Array>> = columns
        .into_iter()
        .map(|col| Arc::new(StringArray::from(col)) as Arc<dyn arrow::array::Array>)
        .collect();
    let batch = RecordBatch::try_new(schema.clone(), arrays)?;
    let file = File::create(path)?;
    let mut writer = ArrowWriter::try_new(file, schema, Some(WriterProperties::builder().build()))?;
    writer.write(&batch)?;
    writer.close()?;
    Ok(())
}

// -------- Avro Import/Export --------
pub fn import_avro(path: &str) -> Result<Value, Box<dyn Error>> {
    use avro_rs::{Reader, types::Value as AvroValue};
    use std::fs::File;
    let file = File::open(path)?;
    let reader = Reader::new(file)?;
    let mut rows = Vec::new();
    for value in reader {
        let avro_val: AvroValue = value?;
        let json = avro_to_json(&avro_val);
        rows.push(json);
    }
    Ok(Value::Array(rows))
}

fn avro_to_json(avro: &avro_rs::types::Value) -> Value {
    match avro {
        avro_rs::types::Value::Null => Value::Null,
        avro_rs::types::Value::Boolean(b) => Value::Bool(*b),
        avro_rs::types::Value::Int(i) => Value::Number((*i).into()),
        avro_rs::types::Value::Long(l) => Value::Number((*l).into()),
        avro_rs::types::Value::Float(f) => serde_json::Number::from_f64(*f as f64)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        avro_rs::types::Value::Double(d) => serde_json::Number::from_f64(*d)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        avro_rs::types::Value::Bytes(b) => Value::String(hex::encode(b)),
        avro_rs::types::Value::String(s) => Value::String(s.clone()),
        avro_rs::types::Value::Array(arr) => Value::Array(arr.iter().map(avro_to_json).collect()),
        avro_rs::types::Value::Map(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                obj.insert(k.clone(), avro_to_json(v));
            }
            Value::Object(obj)
        }
        avro_rs::types::Value::Record(fields) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in fields {
                obj.insert(k.clone(), avro_to_json(v));
            }
            Value::Object(obj)
        }
        _ => Value::Null,
    }
}

pub fn export_avro(value: &Value, path: &str) -> Result<(), Box<dyn Error>> {
    use avro_rs::{Writer, Schema, types::Value as AvroValue};
    use std::fs::File;
    let arr = value.as_array().ok_or("Expected array for Avro export")?;
    if arr.is_empty() {
        return Err("No rows found for Avro export".into());
    }
    let obj = arr[0].as_object().ok_or("Expected object rows in array")?;
    let fields: Vec<_> = obj.keys().map(|k| format!("{{\"name\":\"{}\",\"type\":\"string\"}}", k)).collect();
    let schema_str = format!("{{\"type\":\"record\",\"name\":\"row\",\"fields\":[{}]}}", fields.join(","));
    let schema = Schema::parse_str(&schema_str)?;
    let file = File::create(path)?;
    let mut writer = Writer::new(&schema, file);
    for row in arr {
        let obj = row.as_object().ok_or("Expected object row")?;
        let avro_fields: Vec<(String, AvroValue)> = obj.iter().map(|(k, v)| (k.clone(), AvroValue::String(v.to_string()))).collect();
        writer.append(AvroValue::Record(avro_fields))?;
    }
    writer.flush()?;
    Ok(())
}

// -------- MsgPack Import/Export --------
pub fn import_msgpack(path: &str) -> Result<Value, Box<dyn Error>> {
    use std::fs::File;
    let mut file = File::open(path)?;
    let value: Value = rmp_serde::from_read(&mut file)?;
    Ok(value)
}

pub fn export_msgpack(value: &Value, path: &str) -> Result<(), Box<dyn Error>> {
    use std::fs::File;
    let mut file = File::create(path)?;
    rmp_serde::encode::write(&mut file, value)?;
    Ok(())
}

// -------- CBOR Import/Export --------
pub fn import_cbor(path: &str) -> Result<Value, Box<dyn Error>> {
    use std::fs::File;
    let mut file = File::open(path)?;
    let value: Value = serde_cbor::from_reader(&mut file)?;
    Ok(value)
}

pub fn export_cbor(value: &Value, path: &str) -> Result<(), Box<dyn Error>> {
    use std::fs::File;
    let mut file = File::create(path)?;
    serde_cbor::to_writer(&mut file, value)?;
    Ok(())
}

/// -------- Importers (to serde_json::Value) --------
pub fn import_json(path: &str) -> Result<Value, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

pub fn import_yaml(path: &str) -> Result<Value, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    Ok(serde_yaml::from_str(&data)?)
}

pub fn import_toml(path: &str) -> Result<Value, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let toml_val: toml::Value = toml::from_str(&data)?;
    let json = serde_json::to_value(toml_val)?;
    Ok(json)
}

pub fn import_csv(path: &str) -> Result<Value, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let headers = rdr.headers()?.clone();
    let mut rows = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut map = serde_json::Map::new();
        for (h, v) in headers.iter().zip(record.iter()) {
            map.insert(h.to_string(), Value::String(v.to_string()));
        }
        rows.push(Value::Object(map));
    }
    Ok(Value::Array(rows))
}

/// -------- Exporters (from serde_json::Value) --------
pub fn export_json(value: &Value, path: &str) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(value)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn export_yaml(value: &Value, path: &str) -> Result<(), Box<dyn Error>> {
    let data = serde_yaml::to_string(value)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn export_toml(value: &Value, path: &str) -> Result<(), Box<dyn Error>> {
    let toml_val: toml::Value = serde_json::from_value(value.clone())?;
    let data = toml::to_string_pretty(&toml_val)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn export_csv(value: &Value, path: &str) -> Result<(), Box<dyn Error>> {
    let arr = value.as_array().ok_or("Expected array for CSV export")?;
    if arr.is_empty() {
        return Err("No rows found for CSV export".into());
    }

    let obj = arr[0].as_object().ok_or("Expected object rows in array")?;
    let headers: Vec<String> = obj.keys().cloned().collect();
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(&headers)?;

    for row in arr {
        let obj = row.as_object().ok_or("Expected object row")?;
        let record: Vec<String> = headers
            .iter()
            .map(|h| obj.get(h).map(|v| v.to_string()).unwrap_or_default())
            .collect();
        wtr.write_record(&record)?;
    }

    wtr.flush()?;
    Ok(())
}

// -------- Conversion engine --------
pub fn convert(input: &str, output: &str, from: Format, to: Format) -> Result<(), Box<dyn Error>> {
    // Step 1: Import into serde_json::Value
    let value = match from {
        Format::Json => import_json(input)?,
        Format::Yaml => import_yaml(input)?,
        Format::Toml => import_toml(input)?,
        Format::Csv  => import_csv(input)?,
        Format::Parquet => import_parquet(input)?,
        Format::Avro => import_avro(input)?,
        Format::MsgPack => import_msgpack(input)?,
        Format::Cbor => import_cbor(input)?,
        Format::Ndjson => import_ndjson(input)?,
    };

    // Step 2: Export to target format
    match to {
        Format::Json => export_json(&value, output)?,
        Format::Yaml => export_yaml(&value, output)?,
        Format::Toml => export_toml(&value, output)?,
        Format::Csv  => export_csv(&value, output)?,
        Format::Parquet => export_parquet(&value, output)?,
        Format::Avro => export_avro(&value, output)?,
        Format::MsgPack => export_msgpack(&value, output)?,
        Format::Cbor => export_cbor(&value, output)?,
        Format::Ndjson => export_ndjson(&value, output)?,
    }

    Ok(())
}
