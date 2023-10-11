pub struct Passenger {
    pub name: String,
    pub passport_number: String,
    pub age: u8,
}

pub struct Flight {
    pub flight_code: String,
    pub departure: String,
    pub destination: String,
    pub departure_date: String,
    pub departure_time: String,
}

pub fn new_flight(flight_data: &mut Vec<Flight>, flight_code: String, departure: String, destination: String, departure_date: String, departure_time: String) {
    let flight = Flight {
        flight_code,
        departure,
        destination,
        departure_date,
        departure_time,
    };
    flight_data.push(flight);
}

pub fn new_passenger(passenger_data: &mut Vec<Passenger>, name: String, passport_number: String, age: u8) {
    let passenger = Passenger {
        name,
        passport_number,
        age,
    };
    passenger_data.push(passenger);
}

pub fn show_passengers(passenger_data: Vec<Passenger>) {
    for passenger in passenger_data {
        println!("Passenger's Name: {} | Passport Number: {}", passenger.name, passenger.passport_number);
    }
}
