// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32

}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        validate_product_name(&product_name);
        validate_quantity(quantity);
        validate_unit_price(price);

    }
    pub fn total(self) -> u32 {

    }
    pub fn product_name(self) -> String {
        self.product_name
    }
    pub fn quantity(self) -> u32 {
        self.quantity
    }
    pub fn unit_price(self) -> u32 {
        self.unit_price
    }

    pub fn set_product_name() {

    }
    pub fn set_quantity() {

    }
    pub fn set_unit_price() {

    }
    
}

pub fn validate_product_name(name: &String) {
    if name.is_empty() {
        panic!("name cant be empty");
    }
    if name.len() > 300 {
        panic!("name cant be longer than 300 bytes");
    }

}

pub fn validate_quantity(quantity: u32) {
    if quantity <= 0 {
        panic!("quantity cant be 0 or less");
    }
    quantity

}

pub fn validate_unit_price(price: u32) {
    if price <= 0 {
        panic!("price must be more than 0")
    }
    price

}