extern crate nan;
use nan::*;

fn main() {
    let f = NaNBox::from_f64(3.14);
    println!("Type: {:?}", f.get_type());
    println!("Is f64: {}", f.is_f64());
    println!("Is u64: {}", f.is_u64());
    println!("Is ptr: {}", f.is_ptr());
    println!("Double value: {}", f.as_f64());
    assert!(f.get_type() == Ok(Type::F64));

    let i = NaNBox::from_u64(42);
    println!("Type: {:?}", i.get_type());
    println!("Is f64: {}", i.is_f64());
    println!("Is u64: {}", i.is_u64());
    println!("Is ptr: {}", i.is_ptr());
    println!("U64 value: {}", i.as_u64());
    assert!(i.get_type() == Ok(Type::U64));

    let p = NaNBox::from_ptr(0x1234 as *mut u8);
    println!("Type: {:?}", p.get_type());
    println!("Is f64: {}", p.is_f64());
    println!("Is u64: {}", p.is_u64());
    println!("Is ptr: {}", p.is_ptr());
    println!("Ptr value: {:?}", p.as_ptr());
    assert!(p.get_type() == Ok(Type::Ptr));

    println!("OK")
}
