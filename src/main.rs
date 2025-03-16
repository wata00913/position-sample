mod position;
use position::*;
mod int_position;
use int_position::IntPosition;
mod float_position;
use float_position::FloatPosition;

fn main() {
    test_int_position();
    test_float_position();
}

fn test_float_position() {
    test_from_keys::<FloatPosition>();
    test_add::<FloatPosition>();
    test_insert::<FloatPosition>();
    test_shift::<FloatPosition>();
    test_delete::<FloatPosition>();
}

fn test_int_position() {
    test_from_keys::<IntPosition>();
    test_add::<IntPosition>();
    test_insert::<IntPosition>();
    test_shift::<IntPosition>();
    test_delete::<IntPosition>();
}
