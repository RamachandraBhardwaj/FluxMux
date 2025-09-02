
use async_trait::async_trait;
use fluxmux_core::traits::Codec;
use anyhow::Result;

pub struct JsonCodec;

#[async_trait]
impl Codec for JsonCodec {
    async fn decode(&self, payload: &[u8]) -> Result<serde_json::Value> {
        let v = serde_json::from_slice(payload)?;
        Ok(v)
    }
    async fn encode(&self, v: &serde_json::Value) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(v)?)
    }
}

pub struct CsvCodec;

#[async_trait]
impl Codec for CsvCodec {
    async fn decode(&self, payload: &[u8]) -> Result<serde_json::Value> {
        let mut rdr = csv::Reader::from_reader(payload);
        let mut rows = Vec::new();
        for result in rdr.deserialize() {
            let record: serde_json::Value = result?;
            rows.push(record);
        }
        Ok(serde_json::Value::Array(rows))
    }
    async fn encode(&self, v: &serde_json::Value) -> Result<Vec<u8>> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        if let serde_json::Value::Array(arr) = v {
            for item in arr {
                wtr.serialize(item)?;
            }
        } else {
            wtr.serialize(v)?;
        }
        wtr.flush()?;
        Ok(wtr.into_inner()?)
    }
}
