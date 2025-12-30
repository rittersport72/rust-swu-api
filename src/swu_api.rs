use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct VersionInfo {
    version: String,
    publication_date: String,
    current_status: String,
    last_update: String,
    next_update: Option<String>,
    next_version: Option<String>,
    change_log: String,
    base_url: String,
    used_codes: UsedCodes,
    available_data: AvailableData,
}

#[derive(Serialize, Deserialize, Debug)]
struct UsedCodes {
    // TODO make it an enum
    route_category: std::collections::HashMap<String, String>,
    vehicle_category: std::collections::HashMap<String, String>,
    passage_status: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AvailableData {
    base_data: BaseData,
    //live_data: LiveData, TODO
}

#[derive(Serialize, Deserialize, Debug)]
struct BaseData {
    update_interval: String,
    vehicle: Vehicle,
    route: Route,
    stop: Stop,
    stop_point: StopPoint,
}

#[derive(Serialize, Deserialize, Debug)]
struct Vehicle {
    url: String,
    parameters_get: VehicleParametersGET,
    provided_data_extended: VehicleProvidedDataExtended,
}

#[derive(Serialize, Deserialize, Debug)]
struct VehicleParametersGET {
    vehicle_number: VehicleNumber,
    content_scope: ContentScope,
}

#[derive(Serialize, Deserialize, Debug)]
struct VehicleNumber {
    required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentScope {
    required: bool,
    values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VehicleProvidedDataExtended {
    vehicle_number: String,
    vehicle_category: String,
    vehicle_name: String,
    model_data: ModelData,
    capacity_data: CapacityData,
    technical_data: TechnicalData,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelData {
    make: String,
    model: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CapacityData {
    seats: String,
    standing_spaces: String,
    wheel_chair_spaces: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TechnicalData {
    doors: String,
    length: String,
    width: String,
    height: String,
    empty_weight: String,
    maximum_weight: String,
    power: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Route {
    url: String,
    parameters_get: RouteParametersGET,
    provided_data_extended: RouteProvidedDataExtended,
}

#[derive(Serialize, Deserialize, Debug)]
struct RouteParametersGET {
    route_number: RouteNumber,
    content_scope: ContentScope,
}

#[derive(Serialize, Deserialize, Debug)]
struct RouteNumber {
    required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct RouteProvidedDataExtended {
    route_number: String,
    route_name: String,
    route_category: String,
    route_directions: Vec<RouteDirection>,
    route_pattern: RoutePattern,
}

#[derive(Serialize, Deserialize, Debug)]
struct RouteDirection {
    direction: String,
    direction_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RoutePattern {
    pattern_direction: String,
    stop_points: Vec<StopPointInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopPointInfo {
    stop_number: String,
    stop_code: String,
    stop_name: String,
    stop_point_number: String,
    stop_point_code: String,
    platform_name: String,
    stop_point_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stop {
    url: String,
    parameters_get: StopParametersGET,
    provided_data_extended: StopProvidedDataExtended,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopParametersGET {
    stop_number: StopNumber,
    content_scope: ContentScope,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopNumber {
    required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopProvidedDataExtended {
    stop_number: String,
    stop_code: String,
    stop_name: String,
    stop_announcement: String,
    stop_coordinates: StopCoordinates,
    stop_points: Vec<StopPointInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopCoordinates {
    longitude: String,
    latitude: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopPoint {
    url: String,
    parameters_get: StopPointParametersGET,
    provided_data_extended: StopPointProvidedDataExtended,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopPointParametersGET {
    stop_point_code: StopPointCode,
    content_scope: ContentScope,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopPointCode {
    required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct StopPointProvidedDataExtended {
    stop_point_number: String,
}
