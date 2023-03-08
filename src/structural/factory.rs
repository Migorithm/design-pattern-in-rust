#![allow(dead_code,unused_variables,unused_imports)]

//Abstract Factory and Abstract Products
mod gui{
    pub trait Button{
        fn press(&self);
    }

    pub trait CheckBox{
        fn switch(&self);
    }

    /// Abstract Factory defined using generics
    pub trait GuiFactory{
        type B: Button; // generic
        type C: CheckBox;

        fn create_button(&self)-> Self::B;
        fn create_checkbox(&self)-> Self::C;
    }

    pub trait GuiFactoryDynamic{
        fn create_button(&self) -> Box<dyn Button>;
        fn create_checkbox(&self) -> Box<dyn CheckBox>;
    }
}

pub mod window{
    use super::gui::{GuiFactory,Button,CheckBox,GuiFactoryDynamic};
    
    #[derive(Debug)]
    pub struct WindowButton;

    #[derive(Debug)]
    pub struct WindowCheckbox;

    pub struct WindowFactory;

    impl Button for WindowButton{
        fn press(&self) {
            println!("Window button pressed!, {self:?}");
        }
    }
    impl CheckBox for WindowCheckbox{
        fn switch(&self) {
            println!("Window checkbox switched!, {self:?}")
        }
    }

    impl GuiFactory for WindowFactory{
        type B = WindowButton;
        type C = WindowCheckbox;
        fn create_button(&self)-> Self::B{
            WindowButton
        }
        fn create_checkbox(&self)-> Self::C {
            WindowCheckbox
        }
        
    }


}


pub mod render{
    use super::gui::{GuiFactory,Button,CheckBox,GuiFactoryDynamic};

    /*
    Client code with static dispatch
    Here, the abstract factory is implemented via generics which lets 
    the compiler create a code that does NOT require dynamic dispatch in runtime.
     */
    pub fn render(factory: impl GuiFactory){
        let button1 = factory.create_button();
        let button2 = factory.create_button();
        let checkbox1 = factory.create_checkbox();
        let checkbox2 = factory.create_checkbox();

        button1.press();
        button2.press();
        checkbox1.switch();
        checkbox2.switch();

    }

    pub fn dyn_render(factory: &dyn GuiFactoryDynamic) {
        let button1 = factory.create_button();
        let button2 = factory.create_button();
        let checkbox1 = factory.create_checkbox();
        let checkbox2 = factory.create_checkbox();
    
        button1.press();
        button2.press();
        checkbox1.switch();
        checkbox2.switch();
    }


}