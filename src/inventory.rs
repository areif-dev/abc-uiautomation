/// An *item* represents a product or some other inventory item
#[derive(Debug, PartialEq)]
pub struct Item {
    /// The unique identifier for the item
    pub sku: String,

    /// Briefly describe what the product is
    pub description: Option<String>,

    /// Universal Product Code of the item
    pub upc: Option<String>,

    /// The price of the `Item` for the customer. This appears on `Invoice`s. AKA retail
    pub list: f32,

    /// This is the what the business pays for the `Item`. AKA wholesale
    pub cost: f32,

    /// The unique identifier for the vendor who provides the `Item`
    pub vendor_id: Option<String>,
}

impl Item {
    /// Create a default `Item` with no useful data for an `Item`
    ///
    /// # Returns
    ///
    /// An `Item` with fields: { sku: "", description: "", upc: "", list: 0, cost: 0, vendor_id: "" }
    pub fn new() -> Self {
        Item {
            sku: "".to_string(),
            description: None,
            upc: None,
            list: 0f32,
            cost: 0f32,
            vendor_id: None,
        }
    }
}
