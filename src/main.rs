mod position;
use position::*;
mod int_position;
use int_position::IntPosition;

fn main() {
    test_from_keys::<IntPosition>();
    test_add::<IntPosition>();
    test_insert::<IntPosition>();
    test_shift::<IntPosition>();
    test_delete::<IntPosition>();
}
