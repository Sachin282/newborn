// newborn

mod disturbance;
mod isf;

use disturbance::Disturbance;
use isf::InternalStateField;

fn main() {
    let mut isf = InternalStateField::new();   

    let experiences = vec![
        Disturbance::new(0.9, 0.1, 0.9), //sudden sock on birth
        Disturbance::new(0.3, 2.0, 0.1), //stablizing
        Disturbance::new(0.4, 1.5, 0.2), //stablized : calm, long (rest)
    ];

    for d in experiences {
        isf.apply_dicturbance(&d);
        println!("internal state field: {:?}\n", isf);
    }
}