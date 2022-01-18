use super::ElmExport;

#[allow(dead_code)]
enum RemoteMessage {
    Hello(String),
    Compare(u32, u32),
    Goodbye,
}

impl ElmExport for RemoteMessage {}
