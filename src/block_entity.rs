use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Represents a simplified NBT structure for Block Entities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NbtValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    List(Vec<NbtValue>),
    Compound(HashMap<String, NbtValue>),
    ByteArray(Vec<i8>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

/// Translator for Block Entity NBT data between Bedrock and Java
pub struct BlockEntityTranslator;

impl BlockEntityTranslator {
    /// Translate Bedrock Block Entity NBT to Java format
    pub fn translate_bedrock_to_java(nbt: &HashMap<String, NbtValue>) -> HashMap<String, NbtValue> {
        let mut java_nbt = nbt.clone();
        
        // 1. Identify Block Entity Type
        // Bedrock usually has an "id" string, e.g. "Chest", "Comparator"
        if let Some(NbtValue::String(id)) = nbt.get("id") {
            // Apply specific transformers based on ID
            match id.as_str() {
                "Chest" | "Barrel" | "ShulkerBox" | "TrappedChest" => {
                    Self::translate_container(&mut java_nbt);
                },
                "Comparator" => {
                    Self::translate_comparator(&mut java_nbt);
                },
                _ => {}
            }
        }

        // 2. Generic Item Translation (scan for "Items" list)
        if let Some(NbtValue::List(items)) = java_nbt.get_mut("Items") {
            for item in items {
                if let NbtValue::Compound(item_tag) = item {
                    Self::translate_item(item_tag);
                }
            }
        }

        java_nbt
    }

    fn translate_container(_nbt: &mut HashMap<String, NbtValue>) {
        // Bedrock containers use "Items" just like Java, but the item format differs
        // Handled by generic item translation below
    }

    fn translate_comparator(_nbt: &mut HashMap<String, NbtValue>) {
        // Bedrock: OutputSignal (Int)
        // Java: OutputSignal (Int)
        // Usually pass-through, but verifying
    }

    fn translate_item(item: &mut HashMap<String, NbtValue>) {
        // Bedrock uses "Name" for ID, Java uses "id"
        if let Some(NbtValue::String(name)) = item.remove("Name") {
            let java_id = if name.contains(':') {
                name // Already has namespace
            } else {
                format!("minecraft:{}", name)
            };
            item.insert("id".to_string(), NbtValue::String(java_id));
        }
        
        // Bedrock "Damage" (Short) often maps to Java "Damage" in tag, or metadata
        // For basic items, if there is a "Damage" key, move it to "tag.Damage" for tools?
        // Or is it metadata?
        // In modern Java (1.20.5+), damage is a component.
        // For now, let's keep it simple: Map Name -> id.
    }
}
