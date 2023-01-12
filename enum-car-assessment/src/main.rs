//Practice assessment given in enum ppt
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    // todo!("Fix enum definition so code compiles");
    Manual,
    SemiAuto,
    Automatic,
}
// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {

    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car {
        color: color,
        transmission: transmission,
        convertible: convertible,
        mileage: 0
    }
}
fn main() {
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("1. Car - Color : {}, Transmissiom : {:?}, Convertible : {}, Mileage : {}",car.color, car.transmission, car.convertible, car.mileage);
    
    car = car_factory(String::from("Black"), Transmission::Automatic, true);
    println!("2. Car - Color : {}, Transmissiom : {:?}, Convertible : {}, Mileage : {}",car.color, car.transmission, car.convertible, car.mileage);
    
    car = car_factory(String::from("Silver"), Transmission::SemiAuto, false);
    println!("3. Car - Color : {}, Transmissiom : {:?}, Convertible : {}, Mileage : {}",car.color, car.transmission, car.convertible, car.mileage);
}