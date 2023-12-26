pub mod accounts_receivable;

use std::{thread, time};
use uiautomation::{types::UIProperty, UIAutomation, UIMatcher, UITreeWalker};

pub use uiautomation::UIElement;

pub const SHORT_WAIT_MS: u64 = 100;

/// Convenience function that wraps `UIAutomation.create_matcher()`. Sets `from` to the root
/// element, and sets the `timeout` to `SHORT_WAIT_MS * 30`
///
/// # Arguments
///
/// * `automation` - Reference to the `UIAutomation` struct to create the matcher on
///
/// # Returns
///
/// If successful, return the `UIMatcher`. Will return `Err(uiautomation::Error)` if the root
/// element cannot be found
///
/// # Errors
///
/// If the root element cannot be found, return `Err(uiautomation::Error)`
fn create_matcher_wrapper(automation: &UIAutomation) -> uiautomation::Result<UIMatcher> {
    Ok(automation
        .create_matcher()
        .from(automation.get_root_element()?)
        .timeout(SHORT_WAIT_MS * 30))
}

/// Convenience wrapper around `std::thread::sleep` that pauses the thread for a
/// given number of milliseconds
///
/// # Arguments
///
/// * `duration_ms` - The number of milliseconds the thread should sleep for
pub fn wait(duration_ms: u64) {
    thread::sleep(time::Duration::from_millis(duration_ms));
}

/// Attempt to find and return the active ABC Client4 window
///
/// # Returns
///
/// Will return the ABC Client4 `UIElement` if successful. If Client4 is not already open, return
/// `uiautomation::Error`
///
/// # Errors
///
/// Will return `Err(uiautomation::Error)` if the Client4 window cannot be found
pub fn ensure_abc() -> uiautomation::Result<UIElement> {
    let automation = UIAutomation::new()?;
    create_matcher_wrapper(&automation)?
        .contains_name("ABC Accounting Client")
        .find_first()
}

/// Control ABC Client4 to generate a 323 report (CUSTOMER INVOICE PAYMENTS)
///
/// # Arguments
///
/// * `abc_window` - The `UIElement` representing the Client4 window
/// * `starting_invoice` - The first invoice to send to send to the 323
/// * `ending_invoice` - The last invoice to send to the 323 report
///
/// # Returns
///
/// Will return unit type if successful. Return `uiautomation::Error` if UI manipulation fails at
/// any point
///
/// # Errors
///
/// Will return `Err(uiautomation::Error)` if UI manipulation fails at any point
pub fn generate_report_323(
    abc_window: &UIElement,
    starting_invoice: u64,
    ending_invoice: u64,
) -> uiautomation::Result<()> {
    abc_window.send_keys("{F10}3", SHORT_WAIT_MS * 3)?;
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys("23{enter}", SHORT_WAIT_MS / 2)?;
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys(
        &format!(
            "{{enter}}{}{{enter}}{}{{enter}}t",
            starting_invoice, ending_invoice
        ),
        SHORT_WAIT_MS / 2,
    )?;
    Ok(())
}

/// Control ABC Client4 to generate a 311 report (CUSTOMER INVOICE LEDGER)
///
/// # Arguments
///
/// * `abc_window` - The `UIElement` representing the Client4 window
/// * `starting_invoice` - The first invoice to send to send to the 311
/// * `ending_invoice` - The last invoice to send to the 311 report
///
/// # Returns
///
/// Will return unit type if successful. Return `uiautomation::Error` if UI manipulation fails at
/// any point
///
/// # Errors
///
/// Will return `Err(uiautomation::Error)` if UI manipulation fails at any point
pub fn generate_report_311(
    abc_window: &UIElement,
    starting_invoice: u64,
    ending_invoice: u64,
) -> uiautomation::Result<()> {
    abc_window.send_keys("{F10}3", SHORT_WAIT_MS * 3)?;
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys("11{enter}{enter}{enter}", SHORT_WAIT_MS / 2)?;
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys(
        &format!(
            "{{enter}}{}{{enter}}{}{{enter}}t",
            starting_invoice, ending_invoice
        ),
        SHORT_WAIT_MS / 2,
    )?;
    Ok(())
}

/// Print the tree of elements starting with the first instace of `element` to the last branch
///
/// # Arguments
///
/// * `walker` - Instance of `UITreeWalker` that traverses the tree of elements
/// * `element` - The `UIElement` to start printing from
/// * `level` - How many levels deep into the tree the function is. This is used to print
/// offsetting spaces, so the output appears as a hierarchy
///
/// # Returns
///
/// If successful, return unit type. If a failure occurs, return `uiautomation::Error`
///
/// # Errors
///
/// Will return `Err(uiautomation::Error)` if an element cannot be found
pub fn print_element(
    walker: &UITreeWalker,
    element: &UIElement,
    level: usize,
) -> uiautomation::Result<()> {
    for _ in 0..level {
        print!(" ")
    }
    println!(
        "classname: '{}', name: '{}', value: '{}'",
        element.get_classname()?,
        element.get_name()?,
        element.get_property_value(uiautomation::types::UIProperty::ValueValue)?
    );

    if let Ok(child) = walker.get_first_child(&element) {
        print_element(walker, &child, level + 1)?;

        let mut next = child;
        while let Ok(sibling) = walker.get_next_sibling(&next) {
            print_element(walker, &sibling, level + 1)?;

            next = sibling;
        }
    }

    Ok(())
}

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

/// Send the Ctrl+N key combo to the Client4 window. This may result in a "Save changes before
/// proceeding" popup. If that appears, perform the appropriate action to either save or discard
/// changes based on the value of `save_changes`
///
/// # Arguments
///
/// * `abc_window` - Reference to the Client4 applicaton `UIElement`
/// * `save_changes` - `true` if changes should be saved before a new record is created. `false`
/// otherwise
///
/// # Returns
///
/// Will return `Ok(())` if the function runs successfully. Otherwise, return
/// `Err(uiautomation::Error)` if keypresses fail to send of if the root element cannot be found
///
/// # Errors
///
/// Will return `uiautomation::Error` if any keypresses fail to send or if the root element cannot
/// be found
pub fn send_ctrl_n(abc_window: &UIElement, save_changes: bool) -> uiautomation::Result<()> {
    let automation = UIAutomation::new()?;

    abc_window.hold_send_keys("{Ctrl}", "N", SHORT_WAIT_MS)?;
    wait(SHORT_WAIT_MS);

    // Detect if the "Save changes before proceeding" screen pops up. If it does,
    // perform the appropriate action to either save or discard changes depending on the value of
    // `save_changes`
    let save_changes_popup_result = automation
        .create_matcher()
        .from(automation.get_root_element()?)
        .timeout(SHORT_WAIT_MS / 2)
        .name("Save changes before proceeding?")
        .find_first();
    match (save_changes_popup_result, save_changes) {
        (Ok(popup), true) => {
            popup.send_keys("{enter}", SHORT_WAIT_MS)?;
        }
        (Ok(popup), false) => {
            popup.send_keys("{right}{enter}", SHORT_WAIT_MS)?;
        }
        _ => (), // The popup cannot be found, so there are no changes to save
    }
    Ok(())
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
