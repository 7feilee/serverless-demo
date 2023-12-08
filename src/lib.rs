pub mod lambda_functions {
    use serde_json::{Value, json};
    use lambda_runtime::{Error, Context};

    // Core business logic function
    pub fn process_core_logic(mut event: Value) -> Value {
        if let Some(obj) = event.as_object_mut() {
            obj.insert("processed".to_string(), json!("true"));
        }
        event
    }

    // Lambda handler function
    pub async fn process_data(event: Value, _: Context) -> Result<Value, Error> {
        Ok(process_core_logic(event))
    }
}

#[cfg(test)]
mod tests {
    use super::lambda_functions::process_core_logic;
    use serde_json::json;

    #[test]
    fn test_process_core_logic() {
        let input = json!({"key": "value"});
        let output = process_core_logic(input);
        assert_eq!(output["processed"], "true");
    }
}