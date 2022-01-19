use super::ElmExport;

#[allow(dead_code)]
struct WeatherData {
    position: Coordinate,
    temperature: u64,
    humidity: u64,
}

struct Coordinate {
    latitude: u64,
    longitude: u64,
}

impl ElmExport for WeatherData {}
impl ElmExport for Coordinate {}
