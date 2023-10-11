mod travel;
use travel::{Passenger, Flight, new_flight, new_passenger, show_passengers};

fn main() {
    let mut passenger_data: Vec<Passenger> = Vec::new();
    let mut flight_data: Vec<Flight> = Vec::new();

    new_passenger(&mut passenger_data, String::from("Lara"), String::from("QW1E3R"), 22);
    new_passenger(&mut passenger_data, String::from("Victor"), String::from("K18SDN"), 30);
    new_passenger(&mut passenger_data, String::from("Andressa"), String::from("MZOS13"), 27);

    new_flight(&mut flight_data, String::from("77U16"), String::from("Curitiba"), String::from("Porto Alegre"), String::from("11/10/2023"), String::from("17:00"));
    new_flight(&mut flight_data, String::from("77U16"), String::from("Natal"), String::from("Sao Paulo"), String::from("17/10/2023"), String::from("02:00"));

    show_passengers(passenger_data);
}
