use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
enum JsonPrimitive {
    Number(i32),
    String(String),
}

#[derive(Debug)]
enum JsonValue {
    Number(i32),
    String(String),
    Array(Vec<JsonPrimitive>),
    Map(Json),
}

type Json = HashMap<JsonPrimitive, JsonValue>;

impl PartialEq for Json {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for Json {}

fn main() {
    let json = HashMap::from([
        (JsonPrimitive::Number(1), JsonValue::Number(111)),
        (
            JsonPrimitive::Number(2),
            JsonValue::String("2222".to_owned()),
        ),
        (
            JsonPrimitive::Number(3),
            JsonValue::Array(vec![JsonPrimitive::String("hiya".to_owned())]),
        ),
        (
            JsonPrimitive::String("4".to_owned()),
            JsonValue::Map(HashMap::from([(
                JsonPrimitive::String("5".to_owned()),
                JsonValue::Number(6),
            )])),
        ),
    ]);

    println!("{}", serialize(json));
}

fn serialize(json: Json) -> String {
    let mut serialized_json = "{".to_owned();

    for (key, value) in json {
        let serialized_key = match key {
            JsonPrimitive::Number(n) => n.to_string(),
            JsonPrimitive::String(s) => format!("\"{}\"", s.to_string()),
        };

        let serialized_value = match value {
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", s.to_string()),
            JsonValue::Array(a) => {
                let mut a_str = "[".to_owned();

                let mut vals = a.iter().fold("".to_owned(), |mut s, v| {
                    let formatted_v = match v {
                        JsonPrimitive::Number(n) => n.to_string(),
                        JsonPrimitive::String(s) => format!("\"{}\"", s.to_string()),
                    };
                    s.push_str(&formatted_v);
                    s.push_str(",");
                    s
                });

                // Remove trailing `,`
                vals.pop();

                a_str.push_str(&vals);
                a_str.push_str("]");
                a_str
            }
            JsonValue::Map(m) => serialize(m),
        };

        serialized_json.push_str(&serialized_key);
        serialized_json.push_str(":");
        serialized_json.push_str(&serialized_value);
        serialized_json.push_str(",");
    }

    // Remove trailing `,`
    serialized_json.pop();

    serialized_json.push_str("}");

    serialized_json
}
