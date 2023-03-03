use adder::lib::add;
use adder::question;
#[cfg(test)]
mod tests{ 
    use super::*;    
    #[test]
    fn test_init (){
        assert_eq!( add(4,3),7);
    }

    #[test]
    fn test_init_question(){
        assert_eq!(question::execute(4,3).to_string(),"41.381836");
    }
}
