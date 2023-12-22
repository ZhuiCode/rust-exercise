mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}
        pub fn seat_at_table(){
            crate::front_of_house::serving::take_order();
        }
    }
    mod serving{
        pub fn take_order(){}
        pub fn serve_order(){}
        pub fn take_payment(){}
    }
}
fn server_order(){}
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::server_order();
    }
    fn cook_order(){}
}
pub fn eat_at_restayrant(){
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}