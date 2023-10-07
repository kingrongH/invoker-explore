pub mod invoker_manager;
pub mod invoker;


pub trait Typed {

    fn get<I: 'static>(&self) -> Option<&I>;

    fn get_mut<I: 'static>(&mut self) -> Option<&mut I>;

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
