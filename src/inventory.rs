use bigdecimal::BigDecimal;
use gtin::Gtin;
use uiautomation::{
    errors::{ERR_INACTIVE, ERR_NONE, ERR_NOTFOUND},
    UIAutomation, UIElement,
};

use crate::{
    create_matcher_wrapper, print_element, read_text_box_value, set_text_box_value,
    set_text_box_value_no_enter, wait, SHORT_WAIT_MS,
};

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

/// Control ABC Client4 to navigate to the F10-I screen or the Inventory - Items screen
///
/// # Arguments
/// * `abc_window` - A reference to the ABC Client4 window to control
///
/// # Returns
/// A [`UIElement`] representing the Inventory screen
///
/// #Errors
/// Forwards any automation errors caused by one of the following:
/// * Failing to start an instance of the automation controller
/// * Failing to send keyboard input to ABC
/// * Critical errors while looking for an existing inventory screen
pub fn load_inventory_screen(abc_window: &UIElement) -> uiautomation::Result<UIElement> {
    let automation = UIAutomation::new()?;

    if let Ok(inventory_screen) = create_matcher_wrapper(&automation)?
        .contains_name("Inventory - Items (I)")
        .find_first()
    {
        return Ok(inventory_screen);
    }

    abc_window.send_keys("{F10}I", SHORT_WAIT_MS * 3)?;

    create_matcher_wrapper(&automation)?
        .contains_name("Inventory - Items (I)")
        .find_first()
}

/// Control the inventory screen of ABC to load a specific item by its unique item number
///
/// # Arguments
/// * `inventory_window` - A reference to the Inventory - Items screen of an ABC Client4 instance
/// * `item_number` - The unique identifier for the item to load
///
/// # Errors
/// Forwards any automation errors encountered from any of the following:
/// * Failing to create an instance of the controller
/// * Failing to find, click, or send keyboard input to the input field to enter an item number
pub fn load_item(inventory_window: &UIElement, item_number: &str) -> uiautomation::Result<()> {
    let automation = UIAutomation::new()?;
    let item_num_control = create_matcher_wrapper(&automation)?
        .classname("ThunderRT6TextBox")
        .from(inventory_window.to_owned())
        .find_first()?;
    item_num_control.click()?;
    item_num_control.send_keys(&format!("{}{{enter}}", item_number), SHORT_WAIT_MS)?;
    if &get_sku(inventory_window)? != item_number {
        return Err(uiautomation::Error::new(
            ERR_NONE,
            &format!("could not verify that sku {item_number} was entered successfully"),
        ));
    }
    Ok(())
}

/// Enter the text of `upc` into the UPC input field on an inventory item. Before running this
/// function, you will likely want to call [`load_item`] to set the screen up with an item listing
/// to add `upc` to
///
/// # Arguments
/// * `inventory_window` - The Inventory Items screen of Client4. This screen should ideally have
/// an item listing loaded before calling [`set_upc`]. See [`load_item`] for that
/// * `upc` - The UPC to add to the item listing
///
/// # Errors
/// Forwards any automation errors caused by any of the following:
/// * `inventory_window` is not a reference to the "Inventory - Items" screen of ABC Client4
/// * Failure to create an instance of the controller
/// * Failing to send keyboard input to the UPC text field
/// * Failing to find or send input to the confirmation dialog that pops up to confirm adding a
/// UPC, if it exists.
pub fn set_upc(inventory_window: &UIElement, upc: &Gtin) -> uiautomation::Result<()> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `set_upc`",
        ))?;
    }
    let automation = UIAutomation::new()?;
    set_text_box_value_no_enter(inventory_window, 38, upc.to_string_no_padding())?;
    if get_upc(inventory_window)? != upc.to_string_no_padding() {
        return Err(uiautomation::Error::new(
            ERR_NONE,
            &format!(
                "could not verify that upc {} was set correctly",
                upc.to_string_no_padding()
            ),
        ));
    }
    inventory_window.send_keys("{Enter}", SHORT_WAIT_MS)?;
    wait(SHORT_WAIT_MS * 3);
    if let Ok(confirm) = create_matcher_wrapper(&automation)?
        .classname("ThunderRT6FormDC")
        .contains_name("Add to UPC")
        .find_first()
    {
        confirm.send_keys("y", SHORT_WAIT_MS)?;
    }
    Ok(())
}

pub fn get_upc(inventory_window: &UIElement) -> uiautomation::Result<String> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `get_upc`",
        ))?;
    }
    read_text_box_value(inventory_window, 38)
}

pub fn get_sku(inventory_window: &UIElement) -> uiautomation::Result<String> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `get_sku`",
        ))?;
    }
    read_text_box_value(inventory_window, 0)
}

pub fn get_desc(inventory_window: &UIElement) -> uiautomation::Result<String> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `get_desc`",
        ))?;
    }
    read_text_box_value(inventory_window, 1)
}

pub fn get_list(inventory_window: &UIElement) -> uiautomation::Result<String> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `set_desc`",
        ))?;
    }
    read_text_box_value(inventory_window, 25)
}

pub fn get_cost(inventory_window: &UIElement) -> uiautomation::Result<String> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `get_cost`",
        ))?;
    }

    read_text_box_value(&inventory_window, 26)
}

pub fn get_vendor(inventory_window: &UIElement) -> uiautomation::Result<String> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `get_vendor`",
        ))?;
    }

    read_text_box_value(&inventory_window, 14)
}

pub fn get_sale_gl(inventory_window: &UIElement) -> uiautomation::Result<String> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `get_sale_gl`",
        ))?;
    }

    read_text_box_value(&inventory_window, 43)
}

pub fn get_group(inventory_window: &UIElement) -> uiautomation::Result<String> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `get_group`",
        ))?;
    }

    read_text_box_value(&inventory_window, 39)
}

pub fn get_weight(inventory_window: &UIElement) -> uiautomation::Result<String> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `get_weight`",
        ))?;
    }

    read_text_box_value(&inventory_window, 15)
}

/// Empties the UPC input field of the Inventory Screen. Can be used to entirely delete all UPCs or
/// to just temporarily clear the field in order to add another UPC. Please use [`load_item`]
/// before calling this function in order to have an item to work with.
///
/// # Arguments
/// * `inventory_window` - The Client4 screen titled "Inventory - Items". Should already have an
/// item listing loaded into it by calling [`load_item`]
/// * `delete_fully` - `true` if you would like to remove all UPCs from the listing. `false` if you
/// just want to temporarily clear the input field, eg if you need to add a new UPC
///
/// # Errors
/// Forwards automation errors for any of the following:
/// * `inventory_window` is not a reference to the "Inventory - Items" screen of ABC Client4
/// * Failure to create an instance of the controller
/// * Failing to send keyboard input to the UPC text field
/// * Failure to find or send input to the dialog that pops up to confirm deletion of UPCs
pub fn clear_upc(inventory_window: &UIElement, delete_fully: bool) -> uiautomation::Result<()> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `clear_upc`",
        ))?;
    }
    let automation = UIAutomation::new()?;
    let all_controls = create_matcher_wrapper(&automation)?
        .classname("ThunderRT6TextBox")
        .from(inventory_window.to_owned())
        .find_all()?;
    let Some(upc_control) = all_controls.get(38) else {
        return Err(uiautomation::Error::new(
            ERR_NOTFOUND,
            "Could not find UPC Control on Inventory Screen",
        ))?;
    };
    upc_control.click()?;
    upc_control.send_keys("{Delete}", SHORT_WAIT_MS)?;

    if delete_fully {
        upc_control.send_keys("{Enter}", SHORT_WAIT_MS)?;
        wait(SHORT_WAIT_MS * 3);
        let delete_form = create_matcher_wrapper(&automation)?
            .classname("ThunderRT6FormDC")
            .name("Delete Record")
            .find_first()?;
        delete_form.send_keys("y", SHORT_WAIT_MS)?;
        wait(SHORT_WAIT_MS * 3);
    }
    Ok(())
}

pub fn get_alt_sku(inventory_window: &UIElement, sku_index: u8) -> uiautomation::Result<String> {
    if sku_index > 2 {
        return Err(uiautomation::Error::new(
            ERR_NONE,
            "sku_index must be 0, 1, or 2",
        ));
    }

    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `get_alt_sku`",
        ))?;
    }

    read_text_box_value(inventory_window, 35usize + usize::from(sku_index))
}

pub fn set_alt_sku(
    inventory_window: &UIElement,
    alt_sku: &str,
    sku_index: u8,
) -> uiautomation::Result<()> {
    if sku_index > 2 {
        return Err(uiautomation::Error::new(
            ERR_NONE,
            "sku_index must be 0, 1, or 2",
        ));
    }

    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `set_alt_sku`",
        ))?;
    }
    // Verify that the alt sku was entered correctly. If it wasn't, try again, then fail
    for _ in 0..2 {
        set_text_box_value(
            &inventory_window,
            35_usize + usize::from(sku_index),
            alt_sku,
        )?;
        if get_alt_sku(inventory_window, sku_index)?.trim() == alt_sku.trim() {
            return Ok(());
        }
    }
    return Err(uiautomation::Error::new(
        ERR_NONE,
        "could not verify that alt sku was entered correctly",
    ));
}

pub fn set_desc(inventory_window: &UIElement, desc: &str) -> uiautomation::Result<()> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `set_desc`",
        ))?;
    }

    for _ in 0..2 {
        set_text_box_value(&inventory_window, 1, desc)?;
        if get_desc(inventory_window)?.trim() == desc.trim() {
            return Ok(());
        }
    }
    return Err(uiautomation::Error::new(
        ERR_NONE,
        &format!("could not verify that desc {desc} was entered correctly"),
    ));
}

pub fn set_vendor(inventory_window: &UIElement, vendor: &str) -> uiautomation::Result<()> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `set_vendor`",
        ))?;
    }

    for _ in 0..2 {
        set_text_box_value(&inventory_window, 14, vendor)?;
        if get_vendor(inventory_window)?.trim() == vendor.trim() {
            return Ok(());
        }
    }

    return Err(uiautomation::Error::new(
        ERR_NONE,
        &format!("could not verify that vendor {vendor} was entered correctly"),
    ));
}

pub fn set_weight(inventory_window: &UIElement, weight: f64) -> uiautomation::Result<()> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `set_weight`",
        ))?;
    }

    for _ in 0..2 {
        set_text_box_value(&inventory_window, 15, format!("{:.2}", weight))?;
        if get_weight(inventory_window)? == format!("{:.2}", weight) {
            return Ok(());
        }
    }
    return Err(uiautomation::Error::new(
        ERR_NONE,
        &format!("could not verify that weight {weight} was entered correctly"),
    ));
}

pub fn set_list(inventory_window: &UIElement, list: &BigDecimal) -> uiautomation::Result<()> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `set_list`",
        ))?;
    }

    // If setting the list price fails, try one more time, then fail
    for _ in 0..2 {
        set_text_box_value(&inventory_window, 25, format!("{:.2}", list))?;
        if get_list(inventory_window)? == format!("{:.2}", list) {
            return Ok(());
        }
    }
    return Err(uiautomation::Error::new(
        ERR_NONE,
        "failed to set list price twice",
    ));
}

pub fn set_cost(inventory_window: &UIElement, cost: &BigDecimal) -> uiautomation::Result<()> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `set_cost`",
        ))?;
    }

    for _ in 0..2 {
        set_text_box_value(&inventory_window, 26, format!("{:.2}", cost))?;
        if get_cost(inventory_window)? == format!("{:.2}", cost) {
            return Ok(());
        }
    }
    return Err(uiautomation::Error::new(
        ERR_NONE,
        &format!("could not verify that cost {cost} was entered correctly"),
    ));
}

pub fn set_group(inventory_window: &UIElement, group: &str) -> uiautomation::Result<()> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `set_group`",
        ))?;
    }

    for _ in 0..2 {
        set_text_box_value(&inventory_window, 39, group.to_string())?;
        if &get_group(inventory_window)? == group {
            return Ok(());
        }
    }
    return Err(uiautomation::Error::new(
        ERR_NONE,
        &format!("could not verify that group {group} was entered correctly"),
    ));
}

pub fn set_sale_gl(inventory_window: &UIElement, sale_gl: u32) -> uiautomation::Result<()> {
    if !inventory_window
        .get_name()?
        .starts_with("Inventory - Items (I)")
    {
        return Err(uiautomation::Error::new(
            ERR_INACTIVE,
            "Inventory window is not open in `set_sale_gl`",
        ))?;
    }

    for _ in 0..2 {
        set_text_box_value(&inventory_window, 43, sale_gl.to_string())?;
        if get_sale_gl(inventory_window)?.trim() == sale_gl.to_string().trim() {
            return Ok(());
        }
    }
    return Err(uiautomation::Error::new(
        ERR_NONE,
        &format!("could not verify that sale gl {sale_gl} was entered correctly"),
    ));
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;

    #[test]
    fn test_big_decimal() {
        let price: BigDecimal = "4.99".parse().unwrap();
        assert_eq!(format!("{}", price), String::from("4.99"));
    }
}
