use crate::units::Kg;

///Food Emissions Constants
pub const BEEF: f64 = 27.;
pub const CHICKEN: f64 = 4.56;
pub const PORK: f64 = 6.67;
pub const RICE: f64 = 2.01;
pub const PASTA: f64 = 1.98;
pub const CHEESE: f64 = 6.27;

/// Food Category
#[derive(Debug)]
pub enum Food {
    Chicken { quantity: Kg },
    Beef { quantity: Kg },
    Pork { quantity: Kg },
    Rice { quantity: Kg },
    Pasta { quantity: Kg },
    Cheese { quantity: Kg },
}
