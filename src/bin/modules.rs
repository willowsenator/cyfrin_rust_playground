use cyfrin_playground::math;
use cyfrin_playground::util;
fn main() {
    util::log::debug(&format!("min: {}", math::min(1, 2)));
    util::log::debug(&format!("max: {}", math::max(1, 2)));

    let vec = vec![1, 2, 3];
    if let Some(first) = util::vec::first(&vec) {
        util::log::debug(&format!("First element: {}", first));
    }
    if let Some(last) = util::vec::last(&vec) {
        util::log::debug(&format!("Last element: {}", last));
    }
}