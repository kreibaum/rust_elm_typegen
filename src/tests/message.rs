use serde::Serialize;

use super::ElmExport;

#[allow(dead_code)]
#[derive(Serialize)]
enum RemoteMessage {
    Hello(String),
    Compare(u32, u32),
    Juggle(u32, String, String),
    Goodbye,
}

impl ElmExport for RemoteMessage {}

// Test module, here we just check the serialization of the enum
mod test {
    #[test]
    fn test_variants() {
        use super::RemoteMessage::*;
        let hello_world = serde_json::to_string(&Hello("World".to_string())).unwrap();
        assert_eq!(hello_world, r#"{"Hello":"World"}"#);

        let compare = serde_json::to_string(&Compare(1, 2)).unwrap();
        assert_eq!(compare, r#"{"Compare":[1,2]}"#);

        let juggle =
            serde_json::to_string(&Juggle(1, "Red".to_string(), "Blue".to_string())).unwrap();
        assert_eq!(juggle, r#"{"Juggle":[1,"Red","Blue"]}"#);

        let goodbye = serde_json::to_string(&Goodbye).unwrap();
        assert_eq!(goodbye, r#""Goodbye""#);
    }
}
