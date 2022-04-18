
struct Context {

}

trait Interface {
    
    fn into_type(self) -> InterfaceEnum;

    fn process(&self, context: &mut Context);


}

macro_rules! define_intefaces {
    (
        $(#[$meta:meta])*
        $visual:vis enum $InterfaceName:ident {
            $($variant:ident ($type:ty),)*
        }
    ) => {
        
        $(#[$meta])*
        $visual enum $InterfaceName {
            $($variant($type),)*
        }

        impl Interface for $InterfaceName {
            
            fn into_type(self) -> Self {
                match self {
                    $(
                        $InterfaceName::$variant(p) => {
                            p.into_type()
                        },
                    )*
                }
            }

            fn process(&self, context: &mut Context) {

            }
        }

    };
}

#[derive(Debug)]
pub struct FirstImpl;

impl Interface for FirstImpl {

    fn into_type(self) -> InterfaceEnum {
        InterfaceEnum::First(self)
    }

    fn process(&self, context: &mut Context) {
        // do nothing
    }
    
}


define_intefaces!(
    #[derive(Debug)]
    pub enum InterfaceEnum {
        First(FirstImpl),
    }
);


struct SomeStruct;


impl SomeStruct {

    // some method that wannt to accept generic interface impl
    fn some_method(&self, interface: &InterfaceEnum) {
        
    }

}





