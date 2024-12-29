// Cell 是 provides values,
// if you can cell, use cell.
// aka &mut T
//
// RefCell 是 with references.
// "remove" compiler-time borrow-checks.
// unsafe but check the verification in runtime.
//
use std::cell::{Cell, RefCell};
struct SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>,
}
struct SomeStruct2 {
    regular_field: u8,
    special_field_but_just_ref: RefCell<u8>,
}
struct SomeComplexStruct {
    regular_field: u8,
    special_field: Cell<(u8,i32)>,
    special_field2: RefCell<(u8,i32)>,
}
pub fn run () {
    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };
    let new_value = 100;
    my_struct.special_field.set(new_value);
    assert_eq!(my_struct.special_field.get(), new_value);

    let my_struct2 = SomeStruct2 {
        regular_field: 0,
        special_field_but_just_ref: RefCell::new(1),
    };
    let new_value = 100;
    let mut special_field = my_struct2.special_field_but_just_ref.borrow_mut();
    *special_field = new_value;
    assert_eq!(*special_field, new_value);

    let my_struct3 = SomeComplexStruct {
        regular_field: 0,
        special_field: Cell::new((1,2)),
        special_field2: RefCell::new((1,2)),
    };
    my_struct3.special_field.set((2,3));
    my_struct3.special_field2.borrow_mut().0 = 2;
}
