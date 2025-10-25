use async_trait::async_trait;
use crate::message::{Message, Format};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Trait for pipeline actions that transform messages
#[async_trait]
pub trait PipeAction: Send + Sync {
    async fn execute(&mut self, msg: Message) -> anyhow::Result<Vec<Message>>;
    async fn finalize(&mut self) -> anyhow::Result<Vec<Message>> {
        Ok(vec![])
    }
}

/// Filter action - keeps messages matching expression
pub struct FilterAction {
    expression: String,
}

impl FilterAction {
    pub fn new(expression: String) -> Self {
        Self { expression }
    }

    fn evaluate(&self, value: &Value) -> bool {
        // Simple expression parser: "field>value", "field==value", "field<value"
        let expr = self.expression.trim();
        
        if let Some((field, rest)) = expr.split_once(">=") {
            return self.compare_field(value, field.trim(), rest.trim(), |a, b| a >= b);
        }
        if let Some((field, rest)) = expr.split_once("<=") {
            return self.compare_field(value, field.trim(), rest.trim(), |a, b| a <= b);
        }
        if let Some((field, rest)) = expr.split_once('>') {
            return self.compare_field(value, field.trim(), rest.trim(), |a, b| a > b);
        }
        if let Some((field, rest)) = expr.split_once('<') {
            return self.compare_field(value, field.trim(), rest.trim(), |a, b| a < b);
        }
        if let Some((field, rest)) = expr.split_once("==") {
            return self.compare_field(value, field.trim(), rest.trim(), |a, b| a == b);
        }
        if let Some((field, rest)) = expr.split_once("!=") {
            return self.compare_field(value, field.trim(), rest.trim(), |a, b| a != b);
        }
        
        false
    }

    fn compare_field<F>(&self, value: &Value, field: &str, expected: &str, cmp: F) -> bool
    where
        F: Fn(f64, f64) -> bool,
    {
        if let Some(val) = value.get(field) {
            if let (Some(a), Ok(b)) = (val.as_f64(), expected.parse::<f64>()) {
                return cmp(a, b);
            }
            // String comparison
            if let Some(a) = val.as_str() {
                return match expected.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    Some(b) => a == b,
                    None => a == expected,
                };
            }
        }
        false
    }
}

#[async_trait]
impl PipeAction for FilterAction {
    async fn execute(&mut self, msg: Message) -> anyhow::Result<Vec<Message>> {
        if let Some(ref parsed) = msg.parsed {
            if self.evaluate(parsed) {
                Ok(vec![msg])
            } else {
                Ok(vec![])
            }
        } else {
            Ok(vec![msg])
        }
    }
}

/// Transform action - adds/modifies fields based on expressions
pub struct TransformAction {
    transformations: Vec<(String, String)>, // (field, expression)
}

impl TransformAction {
    pub fn new(expr: String) -> Self {
        let mut transformations = vec![];
        // Parse "field=expr" or "field=expr,field2=expr2"
        for part in expr.split(',') {
            if let Some((field, value_expr)) = part.split_once('=') {
                transformations.push((field.trim().to_string(), value_expr.trim().to_string()));
            }
        }
        Self { transformations }
    }

    fn evaluate_expr(&self, value: &Value, expr: &str) -> Option<Value> {
        // Simple math expressions like "temp*1.8+32"
        let expr = expr.trim();
        
        // Try to evaluate as a simple math expression with field references
        if let Some(result) = self.eval_math(value, expr) {
            return Some(json!(result));
        }
        
        // String literal
        if let Some(s) = expr.strip_prefix('"').and_then(|e| e.strip_suffix('"')) {
            return Some(json!(s));
        }
        
        // Field reference
        if let Some(v) = value.get(expr) {
            return Some(v.clone());
        }
        
        None
    }

    fn eval_math(&self, value: &Value, expr: &str) -> Option<f64> {
        // Very simple evaluator for expressions like "temp*1.8+32"
        let mut result = 0.0;
        let mut current_op = '+';
        let mut current_num = String::new();
        
        for ch in expr.chars() {
            match ch {
                '+' | '-' | '*' | '/' => {
                    if !current_num.is_empty() {
                        let num = if let Ok(n) = current_num.parse::<f64>() {
                            n
                        } else if let Some(v) = value.get(current_num.trim()).and_then(|v| v.as_f64()) {
                            v
                        } else {
                            return None;
                        };
                        
                        result = match current_op {
                            '+' => result + num,
                            '-' => result - num,
                            '*' => if result == 0.0 { num } else { result * num },
                            '/' => if result == 0.0 { num } else { result / num },
                            _ => num,
                        };
                        current_num.clear();
                    }
                    current_op = ch;
                }
                ' ' => {}
                _ => current_num.push(ch),
            }
        }
        
        if !current_num.is_empty() {
            let num = if let Ok(n) = current_num.parse::<f64>() {
                n
            } else if let Some(v) = value.get(current_num.trim()).and_then(|v| v.as_f64()) {
                v
            } else {
                return None;
            };
            
            result = match current_op {
                '+' => result + num,
                '-' => result - num,
                '*' => if result == 0.0 { num } else { result * num },
                '/' => if result == 0.0 { num } else { result / num },
                _ => num,
            };
        }
        
        Some(result)
    }
}

#[async_trait]
impl PipeAction for TransformAction {
    async fn execute(&mut self, mut msg: Message) -> anyhow::Result<Vec<Message>> {
        if let Some(parsed) = msg.parsed.clone() {
            let mut new_value = parsed.clone();
            
            for (field, expr) in &self.transformations {
                if let Some(result) = self.evaluate_expr(&parsed, expr) {
                    if let Some(obj) = new_value.as_object_mut() {
                        obj.insert(field.clone(), result);
                    }
                }
            }
            
            msg.parsed = Some(new_value.clone());
            msg.payload = new_value.to_string().into_bytes();
        }
        
        Ok(vec![msg])
    }
}

/// Aggregate action - groups and aggregates messages
pub struct AggregateAction {
    group_by: Option<String>,
    operations: Vec<(String, String)>, // (operation, field)
    groups: HashMap<String, Vec<Value>>,
}

impl AggregateAction {
    pub fn new(group_by: Option<String>, ops: Vec<(String, String)>) -> Self {
        Self {
            group_by,
            operations: ops,
            groups: HashMap::new(),
        }
    }

    fn get_group_key(&self, value: &Value) -> String {
        if let Some(ref field) = self.group_by {
            value.get(field)
                .and_then(|v| v.as_str())
                .unwrap_or("_default_")
                .to_string()
        } else {
            "_all_".to_string()
        }
    }
}

#[async_trait]
impl PipeAction for AggregateAction {
    async fn execute(&mut self, msg: Message) -> anyhow::Result<Vec<Message>> {
        if let Some(ref parsed) = msg.parsed {
            let key = self.get_group_key(parsed);
            self.groups.entry(key).or_insert_with(Vec::new).push(parsed.clone());
        }
        Ok(vec![]) // Hold messages until finalize
    }

    async fn finalize(&mut self) -> anyhow::Result<Vec<Message>> {
        let mut results = vec![];
        
        for (group_key, values) in &self.groups {
            let mut result_obj = serde_json::Map::new();
            
            if let Some(ref field) = self.group_by {
                result_obj.insert(field.clone(), json!(group_key));
            }
            
            for (op, field) in &self.operations {
                let nums: Vec<f64> = values.iter()
                    .filter_map(|v| v.get(field).and_then(|n| n.as_f64()))
                    .collect();
                
                let result = match op.as_str() {
                    "avg" => nums.iter().sum::<f64>() / nums.len() as f64,
                    "sum" => nums.iter().sum::<f64>(),
                    "min" => nums.iter().cloned().fold(f64::INFINITY, f64::min),
                    "max" => nums.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                    "count" => values.len() as f64,
                    _ => 0.0,
                };
                
                result_obj.insert(format!("{}_{}", op, field), json!(result));
            }
            
            let result_value = Value::Object(result_obj);
            let payload = result_value.to_string().into_bytes();
            
            results.push(Message {
                id: None,
                key: None,
                payload,
                format: Some(Format::Json),
                parsed: Some(result_value),
                timestamp: chrono::Utc::now(),
                headers: Default::default(),
                meta: Default::default(),
            });
        }
        
        Ok(results)
    }
}

/// Normalize action - ensures schema conformance
pub struct NormalizeAction {
    schema: Option<Value>,
}

impl NormalizeAction {
    pub fn new(schema_path: Option<String>) -> Self {
        let schema = schema_path.and_then(|path| {
            std::fs::read_to_string(path).ok()
                .and_then(|content| serde_json::from_str(&content).ok())
        });
        Self { schema }
    }
}

#[async_trait]
impl PipeAction for NormalizeAction {
    async fn execute(&mut self, mut msg: Message) -> anyhow::Result<Vec<Message>> {
        if let (Some(schema), Some(parsed)) = (&self.schema, &msg.parsed) {
            if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
                let mut normalized = serde_json::Map::new();
                
                for (key, _spec) in props {
                    if let Some(val) = parsed.get(key) {
                        normalized.insert(key.clone(), val.clone());
                    }
                }
                
                let normalized_value = Value::Object(normalized);
                msg.parsed = Some(normalized_value.clone());
                msg.payload = normalized_value.to_string().into_bytes();
            }
        }
        Ok(vec![msg])
    }
}

/// Validate action - filters messages that don't match schema
pub struct ValidateAction {
    schema: Option<Value>,
}

impl ValidateAction {
    pub fn new(schema_path: Option<String>) -> Self {
        let schema = schema_path.and_then(|path| {
            std::fs::read_to_string(path).ok()
                .and_then(|content| serde_json::from_str(&content).ok())
        });
        Self { schema }
    }

    fn validate(&self, value: &Value) -> bool {
        if let Some(schema) = &self.schema {
            if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
                if let Some(obj) = value.as_object() {
                    for field in required {
                        if let Some(field_name) = field.as_str() {
                            if !obj.contains_key(field_name) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
}

#[async_trait]
impl PipeAction for ValidateAction {
    async fn execute(&mut self, msg: Message) -> anyhow::Result<Vec<Message>> {
        if let Some(ref parsed) = msg.parsed {
            if self.validate(parsed) {
                Ok(vec![msg])
            } else {
                eprintln!("Validation failed for message");
                Ok(vec![])
            }
        } else {
            Ok(vec![msg])
        }
    }
}

/// Limit action - passes only first N messages
pub struct LimitAction {
    limit: usize,
    count: usize,
}

impl LimitAction {
    pub fn new(limit: usize) -> Self {
        Self { limit, count: 0 }
    }
}

#[async_trait]
impl PipeAction for LimitAction {
    async fn execute(&mut self, msg: Message) -> anyhow::Result<Vec<Message>> {
        if self.count < self.limit {
            self.count += 1;
            Ok(vec![msg])
        } else {
            Ok(vec![])
        }
    }
}

/// Sample action - passes every Nth message
pub struct SampleAction {
    rate: usize,
    count: usize,
}

impl SampleAction {
    pub fn new(rate: usize) -> Self {
        Self { rate, count: 0 }
    }
}

#[async_trait]
impl PipeAction for SampleAction {
    async fn execute(&mut self, msg: Message) -> anyhow::Result<Vec<Message>> {
        self.count += 1;
        if self.count % self.rate == 0 {
            Ok(vec![msg])
        } else {
            Ok(vec![])
        }
    }
}
