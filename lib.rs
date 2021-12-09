#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    #[ink(storage)]
    pub struct Incrementer {
        // ACTION: Create a storage value called `value` which holds a `i32`
        value: i32,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // ACTION: Create a new `Incrementer` and set its `value` to `init_value`
            Self {
                value: init_value,
            }
        }

        // ACTION: Create a second constructor function named `default`.
        //         It has no input, and creates a new `Incrementer` with its `value`
        //         set to `0`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: 0,
            }
        }

        #[ink(message)]
        // ACTION: Update this function to return an i32.
        pub fn get(&self) -> i32 {
            // ACTION: Return the `value`.
            self.value
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            // ACTION: Simply increment `value` by `by`
            
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }
    }
}