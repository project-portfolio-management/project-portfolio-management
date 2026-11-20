//! JSON Schema (the request bodies of the `OpenAPI` document), as the `chrono04` feature of schemars wrote it: inline
//! `{"type":"string","format":"date-time"}` for an instant and `"date"` for a date, under the same schema
//! names and ids, so the `OpenAPI` document does not change. (The `jiff02` feature of schemars is for jiff 0.2.)

use std::borrow::Cow;

use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};

macro_rules! formatted_string {
    ($ty:ty, $name:literal, $id:literal, $format:literal) => {
        impl JsonSchema for $ty {
            fn inline_schema() -> bool {
                true
            }

            fn schema_name() -> Cow<'static, str> {
                $name.into()
            }

            fn schema_id() -> Cow<'static, str> {
                $id.into()
            }

            fn json_schema(_: &mut SchemaGenerator) -> Schema {
                json_schema!({
                    "type": "string",
                    "format": $format
                })
            }
        }
    };
}

formatted_string!(crate::Date, "NaiveDate", "chrono::NaiveDate", "date");
formatted_string!(
    crate::Timestamp,
    "DateTime",
    "chrono::DateTime",
    "date-time"
);
formatted_string!(
    crate::OffsetDateTime,
    "DateTime",
    "chrono::DateTime",
    "date-time"
);

impl JsonSchema for crate::Weekday {
    fn inline_schema() -> bool {
        true
    }

    fn schema_name() -> Cow<'static, str> {
        "Weekday".into()
    }

    fn schema_id() -> Cow<'static, str> {
        "chrono::Weekday".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "enum": ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
        })
    }
}

#[cfg(test)]
mod tests {
    use schemars::JsonSchema;

    #[test]
    fn schemas_are_inline_strings_with_a_format() {
        let date = schemars::schema_for!(crate::Date);
        assert_eq!(date.get("type").and_then(|v| v.as_str()), Some("string"));
        assert_eq!(date.get("format").and_then(|v| v.as_str()), Some("date"));
        let t = schemars::schema_for!(Option<crate::Timestamp>);
        assert_eq!(t.get("format").and_then(|v| v.as_str()), Some("date-time"));

        assert!(crate::Date::inline_schema());
        assert_eq!(crate::Date::schema_name(), "NaiveDate");
        assert_eq!(crate::Date::schema_id(), "chrono::NaiveDate");
        assert!(crate::Timestamp::inline_schema());
        assert_eq!(crate::Timestamp::schema_name(), "DateTime");
        assert_eq!(crate::Timestamp::schema_id(), "chrono::DateTime");
        assert!(crate::OffsetDateTime::inline_schema());
        assert_eq!(crate::OffsetDateTime::schema_name(), "DateTime");
        assert_eq!(crate::OffsetDateTime::schema_id(), "chrono::DateTime");

        let weekday = schemars::schema_for!(crate::Weekday);
        assert!(crate::Weekday::inline_schema());
        assert_eq!(crate::Weekday::schema_name(), "Weekday");
        assert_eq!(crate::Weekday::schema_id(), "chrono::Weekday");
        assert_eq!(
            weekday
                .get("enum")
                .and_then(|value| value.as_array())
                .map(Vec::len),
            Some(7)
        );
    }
}
