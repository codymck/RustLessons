mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String,
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("regular dough"),
                cheese: String::from("mozzarella"),
                topping: String::from(topping),
            }
        }
    }

    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }

        //making help_customer public does not make all child functions public
        pub fn take_order() {
            seat_at_table();
            let cust_pizza: super::Pizza = super::Pizza::lunch("pepperoni");
            serve_customer(cust_pizza);
        }

        fn serve_customer(cust_pizza: super::Pizza) {
            println!(
                "The customer is served a regular pizza with {}",
                cust_pizza.topping
            );
        }
    }
}

pub fn order_food() {
    // how to access all of these functions
    // create a crate of the restaurant folder, that uses the pizza order module, the help customer module and the take order function
    println!("Ordering food");
    crate::restaurant::pizza_order::help_customer::take_order();
}
