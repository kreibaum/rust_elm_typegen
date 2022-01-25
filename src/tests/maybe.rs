use super::ElmExport;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Serialize)]

struct ListMeMaybe {
    list_of_maybe: Vec<Option<i32>>,
    maybe_of_list: Option<Vec<SomeDummyStruct>>,
    double_maybe: Option<Option<bool>>,
}

#[allow(dead_code)]
#[derive(Serialize)]
struct SomeDummyStruct {
    latitude: u64,
    longitude: u64,
}

impl ElmExport for ListMeMaybe {}
impl ElmExport for SomeDummyStruct {}

mod test {
    #[test]
    fn test_serialize_as_expected() {
        use super::ListMeMaybe;
        use super::SomeDummyStruct;
        let list_of_maybe = ListMeMaybe {
            list_of_maybe: vec![Some(1), None, Some(2)],
            maybe_of_list: Some(vec![SomeDummyStruct {
                latitude: 1,
                longitude: 2,
            }]),
            double_maybe: Some(Some(true)),
        };
        let serialized = serde_json::to_string(&list_of_maybe).unwrap();
        assert_eq!(
            serialized,
            r#"{"list_of_maybe":[1,null,2],"maybe_of_list":[{"latitude":1,"longitude":2}],"double_maybe":true}"#
        );

        let list_of_maybe = ListMeMaybe {
            list_of_maybe: vec![],
            maybe_of_list: None,
            double_maybe: Some(None),
        };
        let serialized = serde_json::to_string(&list_of_maybe).unwrap();
        assert_eq!(
            serialized,
            r#"{"list_of_maybe":[],"maybe_of_list":null,"double_maybe":null}"#
        );
    }
}
