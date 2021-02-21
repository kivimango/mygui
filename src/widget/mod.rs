
use specs::Entity;

mod label;
pub use self::label::*;

pub trait Widget {
    fn entity() -> Entity;
}


/*trait Property<T> {
    fn get() -> &'a T;
    fn get_mut() -> &'a mut T;
}

impl Property<T> {
   
}

struct StringProperty {
    text: String
}

impl StringProperty {
    fn new(text: String) -> StringProperty {
        StringProperty {
            text
        }
    }

    fn get(&self) -> &String {
        &self.text
    }

    fn get_mut(&mut self) ->&mut String {
        &mut self.text
    }
}
*/