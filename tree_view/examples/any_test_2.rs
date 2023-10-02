use std::any::Any;

trait Speak: Any {
    fn speak(&self);

    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn as_any(&self) -> &dyn Any;
}

struct Thing;
impl Speak for Thing {
    fn speak(&self) {
        println!("Speaking from thing");
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
trait Other {}
impl Other for Thing {}

fn main() {
    let t = Thing;
    t.speak();

    let b: Box<dyn Speak> = Box::new(t);
    b.speak();

    let o: Box<dyn Any> = b.into_any();
    if let Ok(thing) = o.downcast::<Thing>() {
        thing.speak();
    }

    let t = Box::new(Thing);
    t.speak();

    let o: Box<dyn Any> = t.into_any();
    if let Ok(thing) = o.downcast::<Thing>() {
        thing.speak();
    }
}
