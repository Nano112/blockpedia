    /// Bedrock Edition adapter for build script
    pub struct BedrockDataAdapter;

    impl DataSourceAdapter for BedrockDataAdapter {
        fn name(&self) -> &'static str {
            "BedrockData"
        }

        fn fetch_url(&self) -> &'static str {
            // We'll use blockStates.json as primary for properties
            "https://raw.githubusercontent.com/PrismarineJS/minecraft-data/master/data/bedrock/1.21.0/blockStates.json"
        }

        fn parse_data(&self, json_data: &str) -> Result<Vec<UnifiedBlockData>> {
            let parsed: Value =
                serde_json::from_str(json_data).context("Failed to parse Bedrock blockStates.json")?;

            let states_array = parsed
                .as_array()
                .context("Bedrock blockStates.json is not an array")?;

            let mut block_info_map: HashMap<String, (HashMap<String, Vec<String>>, HashMap<String, String>)> = HashMap::new();

            for state_entry in states_array {
                if let Some(state_obj) = state_entry.as_object() {
                    if let Some(name) = state_obj.get("name").and_then(|n| n.as_str()) {
                        let info = block_info_map.entry(name.to_string()).or_insert_with(|| (HashMap::new(), HashMap::new()));
                        
                        if let Some(states) = state_obj.get("states").and_then(|s| s.as_object()) {
                            for (prop_name, prop_val_obj) in states {
                                if let Some(val) = prop_val_obj.get("value") {
                                    let val_str = match val {
                                        Value::Bool(b) => b.to_string(),
                                        Value::Number(n) => n.to_string(),
                                        Value::String(s) => s.clone(),
                                        _ => continue,
                                    };
                                    
                                    // Add to unique values for this property
                                    let values = info.0.entry(prop_name.clone()).or_insert_with(Vec::new);
                                    if !values.contains(&val_str) {
                                        values.push(val_str.clone());
                                    }
                                    
                                    // First seen value becomes the default (heuristic)
                                    info.1.entry(prop_name.clone()).or_insert(val_str);
                                }
                            }
                        }
                    }
                }
            }

            let mut unified_blocks = Vec::new();
            for (name, (properties, default_state)) in block_info_map {
                let id = format!("minecraft:{}", name);
                unified_blocks.push(UnifiedBlockData {
                    id: id.clone(),
                    properties: properties.clone(),
                    default_state: default_state.clone(),
                    transparent: false, // Default, will be updated by metadata if available
                    extra_properties: HashMap::new(),
                    bedrock_id: Some(id),
                    bedrock_properties: Some(properties),
                    bedrock_default_state: Some(default_state),
                });
            }

            Ok(unified_blocks)
        }

        fn validate_structure(&self, json: &Value) -> Result<()> {
            let states_array = json
                .as_array()
                .context("Bedrock data JSON is not a valid array")?;

            if states_array.is_empty() {
                anyhow::bail!("No states found in Bedrock blockStates data");
            }

            Ok(())
        }
    }
