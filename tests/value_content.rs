use openzwave::value_classes::value_id::{DecimalValue, ValueContent};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_decimal_value() {
        assert_eq!(
            DecimalValue {
                value: 230,
                precision: 1,
            }
            .to_string(),
            "23.0"
        );

        assert_eq!(
            DecimalValue {
                value: 2300,
                precision: 2,
            }
            .to_string(),
            "23.00"
        );

        assert_eq!(
            DecimalValue {
                value: 123456,
                precision: 3,
            }
            .to_string(),
            "123.456"
        );

        assert_eq!(
            DecimalValue {
                value: 123,
                precision: 0,
            }
            .to_string(),
            "123"
        );

        assert_eq!(DecimalValue::from_f32(123 as f32, 0).to_string(), "123");
        assert_eq!(
            DecimalValue::from_f32(123.45 as f32, 2).to_string(),
            "123.45"
        );
    }

    #[test]
    fn test_value_content_unsupported() {
        // unsupported type yet
        assert_eq!(ValueContent::Raw.to_string(), "null");
        assert_eq!(ValueContent::Schedule.to_string(), "null");
    }

    #[test]
    fn test_value_content_supported() {
        assert_eq!(ValueContent::String("foo".into()).to_string(), "\"foo\"");
        // Unsure about the List thing.
        assert_eq!(ValueContent::List("foo".into()).to_string(), "\"foo\"");
        assert_eq!(ValueContent::Bool(true).to_string(), "true");
        assert_eq!(ValueContent::Button(true).to_string(), "true");
        assert_eq!(ValueContent::Byte(42).to_string(), "42");
        assert_eq!(ValueContent::Int(0815).to_string(), "815");
        assert_eq!(ValueContent::Int(-42).to_string(), "-42");
        assert_eq!(ValueContent::Short(123).to_string(), "123");
        assert_eq!(ValueContent::Unknown.to_string(), "null");
    }
}
