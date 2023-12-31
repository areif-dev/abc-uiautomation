use crate::{wait, UIElement, SHORT_WAIT_MS};

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
