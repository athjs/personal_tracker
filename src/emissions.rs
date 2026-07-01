use crate::model::food::{BEEF, CHEESE, CHICKEN, PASTA, PORK, RICE};
use crate::model::transport::{
    BIKE, ELECTRICBIKE, ELECTRICBUS, ELECTRICCAR, ELECTRICMOTOCYCLE, ERT, HST, OILCAR, PETROLBUS,
    PETROLCAR, PETROLMOTOCYCLE, PLANE,
};
use crate::model::{Food, FuelType, Transport};
use crate::units::Kg;

/// Emission Category
#[derive(Debug)]
pub enum Category {
    Food,
    Transport,
    Energy,
}

/// Emission Source
pub trait EmissionSource {
    /// Function transforming an action into a CO2 emission in Kg
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
    /// Determines the CO2 emission for a traject in Kg
    ///
    /// # Parameters ;
    ///  - ‘self‘ : An Emission source
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
    /// Category of the Emission : Transport
    ///
    /// # Parameter :
    /// - ‘self‘ : A transport
    fn category(&self) -> Category {
        Category::Transport
    }
}

/// Implementing EmissionSource for Food
impl EmissionSource for Food {
    /// Emission of CO2 in Kg for a Kg of aliment eaten
    ///
    /// # Parameter :
    /// ‘self‘ : Food aliment
    fn co2_to_kg(&self) -> Kg {
        match self {
            Food::Beef { quantity } => Kg(BEEF * quantity.0),
            Food::Pork { quantity } => Kg(PORK * quantity.0),
            Food::Chicken { quantity } => Kg(CHICKEN * quantity.0),
            Food::Rice { quantity } => Kg(RICE * quantity.0),
            Food::Cheese { quantity } => Kg(CHEESE * quantity.0),
            Food::Pasta { quantity } => Kg(PASTA * quantity.0),
        }
    }

    /// Category : Food
    fn category(&self) -> Category {
        Category::Food
    }
}
