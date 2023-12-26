use crate::{create_matcher_wrapper, wait, SHORT_WAIT_MS};
use uiautomation::{types::UIProperty, UIAutomation, UIElement};

/// Control the ABC Client4 window to load the Invoices records screen, and return the `UIElement`
/// that represents that screen.
///
/// # Arguments
///
/// * `abc_window` - Reference to the `UIElement` representing the Client4 window. The easiest way
/// to get this value is to call `abc_controller::load_abc`
///
/// # Returns
///
/// If successful, return the `UIElement` that represents the Invoices screen. This is a control
/// inside Client4 whose name contains "Sales - Invoices (R)"
///
/// If any key combos fail to send or if the invoices screen cannot be found, return
/// `Err(uiautomation::Error)`
///
/// # Errors
///
/// If any key combos fail to send or if the invoices screen cannot be found, return
/// `Err(uiautomation::Error)`
pub fn load_invoices_screen(abc_window: &UIElement) -> uiautomation::Result<UIElement> {
    let automation = UIAutomation::new()?;

    if let Ok(invoices_screen) = create_matcher_wrapper(&automation)?
        .contains_name("Sales - Invoices (R)")
        .find_first()
    {
        return Ok(invoices_screen);
    }

    abc_window.send_keys("{F10}R", SHORT_WAIT_MS * 3)?;

    create_matcher_wrapper(&automation)?
        .contains_name("Sales - Invoices (R)")
        .find_first()
}

/// Loads an invoice identified by its number into the provided invoices window.
///
/// # Arguments
///
/// * `invoices_window` - A reference to a UI element (e.g., ABC (R) screen or Accounts Receivable Screen)
///   containing invoice details.
/// * `invoice_num` - The unique identifier of the invoice to be loaded.
///
/// # Returns
///
/// A `Result` indicating the success or failure of loading the invoice.
/// If successful, returns `Ok(())`.
/// If any UI element is not found or if there are errors during the loading process,
/// returns an `Err` with a specific error code and message.
///
/// # Errors
///
/// * Returns an error if it fails to find the UI element representing the invoice number control.
/// * Returns an error if there are issues during the UI automation process.
///
/// # Examples
///
/// ```rust
/// use uiautomation::UIElement;
///
/// // Assuming `invoices_window` and `invoice_num` are properly initialized
/// let loading_result = load_invoice(&invoices_window, 123);
/// match loading_result {
///     Ok(_) => println!("Invoice successfully loaded."),
///     Err(err) => println!("Error: {}", err),
/// }
/// ```
pub fn load_invoice(invoices_window: &UIElement, invoice_num: u64) -> uiautomation::Result<()> {
    let automation = UIAutomation::new()?;

    let invoice_num_control = create_matcher_wrapper(&automation)?
        .classname("ThunderRT6TextBox")
        .from(invoices_window.to_owned())
        .find_first()?;
    invoice_num_control.click()?;
    invoice_num_control.send_keys(&format!("{}{{enter}}", invoice_num), SHORT_WAIT_MS)?;

    Ok(())
}

/// Sends an invoice to JDF (John Deere Financial) and checks if the operation was successful.
///
/// # Arguments
///
/// * `invoices_window` - A reference to the ABC (R) Accounts Receivable screen. This is most
/// easily obtained by running `load_invoices_screen`
/// * `invoice_num` - The unique identifier of the invoice being sent to JDF.
///
/// # Returns
///
/// A `Result` indicating whether the invoice was successfully sent to JDF (`true`) or not (`false`).
/// If successful, returns `Ok(true)` if the invoice was sent to JDF or `Ok(false)` if not sent.
/// If any UI element is not found or if there are errors during the process, returns an `Err` with a specific error code and message.
///
/// # Errors
///
/// * Returns an error if it fails to find the UI element representing the paid control.
/// * Returns an error if there are issues during the UI automation process.
///
/// # Examples
///
/// ```rust
/// use uiautomation::UIElement;
///
/// // Assuming `invoices_window` and `invoice_num` are properly initialized
/// let jdf_result = send_invoice_to_jdf(&invoices_window, 123);
/// match jdf_result {
///     Ok(true) => println!("Invoice successfully sent to JDF."),
///     Ok(false) => println!("Failed to send invoice to JDF."),
///     Err(err) => println!("Error: {}", err),
/// }
/// ```
pub fn send_invoice_to_jdf(
    invoices_window: &UIElement,
    invoice_num: u64,
) -> uiautomation::Result<bool> {
    let automation = UIAutomation::new()?;

    load_invoice(invoices_window, invoice_num)?;
    let all_text_boxes = create_matcher_wrapper(&automation)?
        .from(invoices_window.to_owned())
        .classname("ThunderRT6TextBox")
        .find_all()?;
    let paid_control = match all_text_boxes.get(29) {
        Some(c) => c,
        None => return Err(uiautomation::Error::new(2, "could not find paid control")),
    };
    let paid_control_value = paid_control
        .get_property_value(UIProperty::ValueValue)?
        .get_string()?;
    if paid_control_value != String::new() {
        return Ok(true);
    }

    invoices_window.send_keys("{F9}7R", SHORT_WAIT_MS * 3)?;
    wait(2000);

    let invoice_num_control = create_matcher_wrapper(&automation)?
        .classname("ThunderRT6TextBox")
        .from(invoices_window.to_owned())
        .find_first()?;
    let invoice_num_control_value = invoice_num_control
        .get_property_value(UIProperty::ValueValue)?
        .get_string()?;
    if invoice_num.to_string() == invoice_num_control_value {
        invoices_window.send_keys("{enter}{esc}", SHORT_WAIT_MS * 3)?;
        invoices_window.hold_send_keys("{ctrl}", "n", SHORT_WAIT_MS * 3)?;
        invoices_window.send_keys("{right}{enter}", SHORT_WAIT_MS * 3)?;
        return Ok(false);
    }

    Ok(true)
}

/// Checks if an invoice is fully paid by comparing values extracted from specific UI elements.
///
/// # Arguments
///
/// * `invoices_window` - A reference to a UI element containing invoice details.
/// * `invoice_num` - The unique identifier of the invoice being checked.
///
/// # Returns
///
/// A `Result` indicating whether the invoice is fully paid (`true`) or not (`false`).
/// If successful, returns `Ok(true)` if the paid amount matches the total amount; otherwise, `Ok(false)`.
/// If any UI element is not found or if there are errors, returns an `Err` with a specific error code and message.
///
/// # Errors
///
/// * Returns an error if it fails to find the UI element representing the paid amount.
/// * Returns an error if it fails to find the UI element representing the total amount.
///
/// # Examples
///
/// ```rust
/// use uiautomation::UIElement;
///
/// // Assuming `invoices_window` and `invoice_num` are properly initialized
/// let is_fully_paid = is_invoice_fully_paid(&invoices_window, 123);
/// match is_fully_paid {
///     Ok(true) => println!("The invoice is fully paid."),
///     Ok(false) => println!("The invoice is not fully paid."),
///     Err(err) => println!("Error: {}", err),
/// }
/// ```
pub fn is_invoice_fully_paid(
    invoices_window: &UIElement,
    invoice_num: u64,
) -> uiautomation::Result<bool> {
    let automation = UIAutomation::new()?;

    load_invoice(invoices_window, invoice_num)?;
    let all_text_boxes = create_matcher_wrapper(&automation)?
        .from(invoices_window.to_owned())
        .classname("ThunderRT6TextBox")
        .find_all()?;
    let paid_control = match all_text_boxes.get(29) {
        Some(c) => c,
        None => return Err(uiautomation::Error::new(2, "could not find paid control")),
    };
    let total_control = match all_text_boxes.get(38) {
        Some(c) => c,
        None => {
            return Err(uiautomation::Error::new(
                2,
                "could not find invoice total control",
            ))
        }
    };
    let paid_control_value = paid_control
        .get_property_value(UIProperty::ValueValue)?
        .get_string()?;
    let total_control_value = total_control
        .get_property_value(UIProperty::ValueValue)?
        .get_string()?;

    Ok(paid_control_value == total_control_value)
}
