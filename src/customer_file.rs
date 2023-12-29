use crate::{create_matcher_wrapper, UIAutomation, UIElement, SHORT_WAIT_MS};
use uiautomation::types::UIProperty;

/// Control the ABC Client4 window to load the Customer records screen, and return the `UIElement`
/// that represents that screen.
///
/// # Arguments
///
/// * `abc_window` - Reference to the `UIElement` representing the Client4 window. The easiest way
/// to get this value is to call `abc_controller::load_abc`
///
/// # Returns
///
/// If successful, return the `UIElement` that represents the Customer screen. This is a control
/// inside Client4 whose name contains "Sales - Customers (C)"
///
/// If any key combos fail to send or if the customer screen cannot be found, return
/// `Err(uiautomation::Error)`
///
/// # Errors
///
/// If any key combos fail to send or if the customer screen cannot be found, return
/// `Err(uiautomation::Error)`
pub fn load_customer_screen(abc_window: &UIElement) -> uiautomation::Result<UIElement> {
    let automation = UIAutomation::new()?;

    if let Ok(customer_screen) = create_matcher_wrapper(&automation)?
        .contains_name("Sales - Customers (C)")
        .find_first()
    {
        return Ok(customer_screen);
    }

    abc_window.send_keys("{F10}C", SHORT_WAIT_MS * 3)?;

    create_matcher_wrapper(&automation)?
        .contains_name("Sales - Customers (C)")
        .find_first()
}

/// Get the JDF account ID for a given customer from a running Client4 window.
///
/// # Arguments
///
/// * `customer_screen` - Reference to the customer window of a Client4 instance. The name of the
/// element should contain "Sales - Customer (C)". The simplest way to get this value is to call
/// `abc_controller::load_customer_screen`, which will return the `UIElement` representing the
/// Customer screen.
///
/// * `customer_code` - The ID of the customer to fetch the JDF code for. Eg. "DOEJO 0"
///
/// # Returns
///
/// If successful, return the JDF account number represented as a `String`. If there is no account
/// number, return `String::new()`.
///
/// If any key combos fail to send, or if the "ThunderRT6TextBox" containing the JDF account number
/// cannot be found, return `Err(uiautomation::Error)`
///
/// # Errors
///
/// Return `Err(uiautomation::Error)` if any key combos fail or the text box containing the JDF
/// account number cannot be found
pub fn jdf_account_by_customer(
    customer_screen: &UIElement,
    customer_code: &str,
) -> uiautomation::Result<String> {
    let automation = UIAutomation::new()?;

    customer_screen.send_keys(
        &format!("{{up}}{}{{enter}}", customer_code),
        SHORT_WAIT_MS / 4,
    )?;

    // The John Deere Account number is the 29th (counting from 1) ThunderRT6TextBox
    let text_boxes = automation
        .create_matcher()
        .from(customer_screen.to_owned())
        .timeout(SHORT_WAIT_MS)
        .classname("ThunderRT6TextBox")
        .find_all()?;
    let jdf_account_text_box = match text_boxes.get(28) {
        Some(b) => b,
        None => return Ok(String::new()),
    };
    let jdf_account_variant = jdf_account_text_box.get_property_value(UIProperty::ValueValue)?;
    jdf_account_variant.get_string()
}

pub fn load_customer_record(
    customer_screen: &UIElement,
    customer_code: &str,
) -> uiautomation::Result<()> {
    let automation = UIAutomation::new()?;

    let customer_code_control = create_matcher_wrapper(&automation)?
        .classname("ThunderRT6TextBox")
        .from(customer_screen.to_owned())
        .find_first()?;
    customer_code_control.click()?;
    customer_code_control.send_keys(&format!("{}{{enter}}", customer_code), SHORT_WAIT_MS)?;

    Ok(())
}
