use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct VersionInfo {
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
#[serde(rename_all = "PascalCase")]
pub struct UsedCodes {
    // TODO make it an enum
    route_category: i32,
    vehicle_category: i32,
    passage_status: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AvailableData {
    base_data: BaseData,
    //live_data: LiveData, TODO
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BaseData {
    update_interval: String,
    vehicle: Vehicle,
    route: Route,
    stop: Stop,
    stop_point: StopPoint,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Vehicle {
    url: String,
    parameters_get: VehicleParametersGET,
    provided_data_extended: VehicleProvidedDataExtended,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct VehicleParametersGET {
    vehicle_number: VehicleNumber,
    content_scope: ContentScope,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct VehicleNumber {
    required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ContentScope {
    required: bool,
    values: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct VehicleProvidedDataExtended {
    vehicle_number: i32,
    vehicle_category: i32,
    vehicle_name: String,
    model_data: ModelData,
    capacity_data: CapacityData,
    technical_data: TechnicalData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ModelData {
    make: String,
    model: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CapacityData {
    seats: i32,
    standing_spaces: i32,
    wheel_chair_spaces: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TechnicalData {
    doors: i32,
    length: i32,
    width: i32,
    height: i32,
    empty_weight: i32,
    maximum_weight: i32,
    power: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Route {
    url: String,
    parameters_get: RouteParametersGET,
    provided_data_extended: RouteProvidedDataExtended,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteParametersGET {
    route_number: i32,
    content_scope: ContentScope,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteNumber {
    required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteProvidedDataExtended {
    route_number: i32,
    route_name: String,
    route_category: i32,
    route_directions: RouteDirections,
    route_pattern: RoutePattern,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RouteDirections {
    direction: i32,
    direction_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RoutePattern {
    pattern_direction: String,
    stop_points: Vec<StopPointInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopPointInfo {
    stop_number: i32,
    stop_code: String,
    stop_name: String,
    stop_point_number: i32,
    stop_point_code: i32,
    platform_name: String,
    stop_point_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Stop {
    url: String,
    parameters_get: StopParametersGET,
    provided_data_extended: StopProvidedDataExtended,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopParametersGET {
    stop_number: i32,
    content_scope: ContentScope,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopNumber {
    required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopProvidedDataExtended {
    stop_number: i32,
    stop_code: String,
    stop_name: String,
    stop_announcement: String,
    stop_coordinates: StopCoordinates,
    stop_points: Vec<StopPointInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopCoordinates {
    longitude: String,
    latitude: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopPoint {
    url: String,
    parameters_get: StopPointParametersGET,
    provided_data_extended: StopPointProvidedDataExtended,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopPointParametersGET {
    stop_point_code: StopPointCode,
    content_scope: ContentScope,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopPointCode {
    required: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopPointProvidedDataExtended {
    stop_point_number: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_direction() {
        let rd = RouteDirections {
            direction: 123,
            direction_name: "street".to_string(),
        };
        // Prints rd
        println!("rd = {:?}", rd);

        // Convert RouteDirection to a JSON string
        let serialized = serde_json::to_string(&rd).unwrap();

        // Prints serialized
        println!("serialized = {}", serialized);

        // Convert the JSON string back to a RouteDirection
        let deserialized: RouteDirections = serde_json::from_str(&serialized.as_str()).unwrap();

        // Prints deserialized
        println!("deserialized = {:?}", deserialized);

        assert_eq!(rd, deserialized);
    }
}
