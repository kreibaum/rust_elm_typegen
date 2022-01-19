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

#[allow(dead_code)]
enum MixedData {
    GoodData(WeatherData),
    BadData(Coordinate),
}

impl ElmExport for WeatherData {}
impl ElmExport for Coordinate {}
impl ElmExport for MixedData {}
