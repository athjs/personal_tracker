use crate::emissions::Category;
use crate::units::Kg;
use std::collections::HashMap;
// use chrono::DateRange;
/// Struct creating the use's report
#[derive(Debug)]
pub struct Report {
    // pub period: DateRange, //FIX: need to find the corresponding crate
    pub by_category: HashMap<Category, Kg>,
    pub total: Kg,
    pub daily_avg: Kg,
}
