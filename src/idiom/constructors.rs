//Rust does not have constructors as a language construct. So convention is to use.

#[derive(Debug,PartialEq, Eq)]
pub struct People{
    name:String,
    age:i32
}

impl People{
    pub fn new(name:&str,age:i32)-> Self{
        Self{name:name.to_string(),age}
    }
}

//Rust also provides default trait
#[derive(Default,Debug,PartialEq)]
pub struct Second{
    value:u64
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_constructor(){
        assert_eq!(People::new("migo",54),People{name:"migo".to_string(),age:54})

    }

    #[test]
    fn test_default_constructor(){
        assert_eq!(Second::default(), Second{value:0})
    }
}