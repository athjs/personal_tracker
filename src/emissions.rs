use crate::units::{Kg, Km};

///Constants of Emissions
const PETROLCAR: f64 = 0.142253;
const OILCAR: f64 = 0.169708;
const BIKE: f64 = 0.00017;
const ELECTRICBIKE: f64 = 0.001095;
const ELECTRICCAR: f64 = 0.067365;
const ELECTRICBUS: f64 = 0.0217;
const PETROLBUS: f64 = 0.03756;
const PLANE: f64 = 0.177894;
const ELECTRICMOTOCYCLE: f64 = 0.059300;
const PETROLMOTOCYCLE: f64 = 0.076300;
const HST: f64 = 0.00293; // High Speed Train
const ERT: f64 = 0.027690; // Extern Region Train

/// Emission Category
#[derive(Debug)]
pub enum Category {
    Food,
    Transport,
    Energy,
}

/// Food Category
#[derive(Debug)]
pub enum Food {
    Chicken,
    Beef,
    Pork,
    Vegetables,
}
///FuelType
#[derive(Debug)]
pub enum FuelType {
    Petrol,
    Oil,
    Electric,
}
/// Mean of Transport for a traject
#[derive(Debug)]
pub enum Transport {
    Car { km: Km, fuel: FuelType },
    Bus { km: Km, fuel: FuelType },
    Plane { km: Km },
    Train { km: Km, fuel: FuelType },
    Bike { km: Km },
    ElectricBike { km: Km },
    Motocycle { km: Km, fuel: FuelType },
}
/// Emission Source
pub trait EmissionSource {
    /// Function transforming a journey to a CO2 emission in KG
    ///
    /// # Parameters :
    /// - ‘self‘ : An Emission Source
    fn co2_to_kg(&self) -> Kg;

    /// Getter for the gatefory of Transport
    ///
    /// # Parameters :
    /// - ‘self‘ : An Emission Source
    fn category(&self) -> Category;
}

/// Implentinig EmissionSource for Transport according to AGIR
impl EmissionSource for Transport {
    fn co2_to_kg(&self) -> Kg {
        match self {
            Transport::Car {
                km,
                fuel: FuelType::Petrol,
            } => Kg(km.0 * PETROLCAR),
            Transport::Car {
                km,
                fuel: FuelType::Oil,
            } => Kg(km.0 * OILCAR),
            Transport::Car {
                km,
                fuel: FuelType::Electric,
            } => Kg(km.0 * ELECTRICCAR),
            Transport::Bike { km } => Kg(km.0 * BIKE),
            Transport::ElectricBike { km } => Kg(km.0 * ELECTRICBIKE),
            Transport::Motocycle {
                km,
                fuel: FuelType::Electric,
            } => Kg(km.0 * ELECTRICMOTOCYCLE),
            Transport::Motocycle {
                km,
                fuel: FuelType::Oil,
            } => Kg(km.0 * PETROLMOTOCYCLE),
            Transport::Motocycle {
                km,
                fuel: FuelType::Petrol,
            } => Kg(km.0 * PETROLMOTOCYCLE),
            Transport::Bus {
                km,
                fuel: FuelType::Petrol,
            } => Kg(km.0 * PETROLBUS),
            Transport::Bus {
                km,
                fuel: FuelType::Oil,
            } => Kg(km.0 * PETROLBUS),
            Transport::Bus {
                km,
                fuel: FuelType::Electric,
            } => Kg(km.0 * ELECTRICBUS),
            Transport::Plane { km } => Kg(km.0 * PLANE),
            Transport::Train {
                km,
                fuel: FuelType::Oil,
            } => Kg(km.0 * ERT),
            Transport::Train {
                km,
                fuel: FuelType::Petrol,
            } => Kg(km.0 * ERT),
            Transport::Train {
                km,
                fuel: FuelType::Electric,
            } => Kg(km.0 * HST),
        }
    }
    fn category(&self) -> Category {
        Category::Transport
    }
}
