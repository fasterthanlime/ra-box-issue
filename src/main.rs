trait Foo {}

fn give_foo() -> Result<Box<dyn Foo>, ()> {
    unimplemented!()
}

fn main() -> Result<(), ()> {
    let _foo = give_foo();
    let _foo = give_foo().unwrap();
    let _foo = give_foo()?;
    Ok(())
}
