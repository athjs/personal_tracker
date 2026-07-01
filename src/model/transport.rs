use crate::units::Km;

/// Transport Emissions Constants
pub const PETROLCAR: f64 = 0.142253;
pub const OILCAR: f64 = 0.169708;
pub const BIKE: f64 = 0.00017;
pub const ELECTRICBIKE: f64 = 0.001095;
pub const ELECTRICCAR: f64 = 0.067365;
pub const ELECTRICBUS: f64 = 0.0217;
pub const PETROLBUS: f64 = 0.03756;
pub const PLANE: f64 = 0.177894;
pub const ELECTRICMOTOCYCLE: f64 = 0.059300;
pub const PETROLMOTOCYCLE: f64 = 0.076300;
pub const HST: f64 = 0.00293; // High Speed Train
pub const ERT: f64 = 0.027690; // Extern Region Train

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
