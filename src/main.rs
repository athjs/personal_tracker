use personal_tracker::units::Kg;
fn main() {
    let twelve = Kg(12.);
    let ten = Kg(10.);
    let twntwo = twelve + ten;
    println!("{twntwo:?}");
}
