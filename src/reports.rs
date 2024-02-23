use crate::{wait, UIElement, SHORT_WAIT_MS};

/// Control ABC Client4 to generate any simple report that follows the pattern:
/// * Open F10 
/// * Enter menu option
/// * Enter report option
/// * Enter Starting at
/// * Enter Ending with
/// * Send output to TabFile
///
/// # Arguments
///
/// * `abc_window` - The `UIElement` representing the Client4 window
/// * `menu` - The string representing the overall report menu to pick from. EG 1 for inventory r
/// reports or 2 for billing reports
/// * `report` - The string representing the specific report to choose from the menu
/// * `starting_at` - The first record to run the report for
/// * `ending_with` - The last record that should be included in the report
///
/// # Returns
///
/// Will return unit type if successful. Return `uiautomation::Error` if UI manipulation fails at
/// any point
///
/// # Errors
///
/// Will return `Err(uiautomation::Error)` if UI manipulation fails at any point
pub fn generate_simple_report(
    abc_window: &UIElement,
    menu: &str,
    report: &str,
    starting_at: &str,
    ending_with: &str,
) -> uiautomation::Result<()> {
    abc_window.send_keys(&format!("{{F10}}{}", menu), SHORT_WAIT_MS * 3)?;
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys(&format!("{}{{enter}}", report_number), SHORT_WAIT_MS / 2)?;
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys(
        &format!("{{enter}}{}{{enter}}{}{{enter}}t", starting_at, ending_with),
        SHORT_WAIT_MS / 2,
    )?;
    Ok(())
}

/// Control ABC Client4 to generate a 1-1 report (INVENTORY LISTING)
///
/// # Arguments
///
/// * `abc_window` - The `UIElement` representing the Client4 window
/// * `starting_sku` - The first sku to send to send to the 11
/// * `ending_sku` - The last sku to send to the 11 report
///
/// # Returns
///
/// Will return unit type if successful. Return `uiautomation::Error` if UI manipulation fails at
/// any point
///
/// # Errors
///
/// Will return `Err(uiautomation::Error)` if UI manipulation fails at any point
pub fn generate_report_11(
    abc_window: &UIElement,
    starting_sku: &str,
    ending_sku: &str,
) -> uiautomation::Result<()> {
    abc_window.send_keys("{F10}1", SHORT_WAIT_MS * 3)?;
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys("1{enter}", SHORT_WAIT_MS / 2)?;
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys("I", SHORT_WAIT_MS / 2)?;
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys(
        &format!("{{enter}}{}{{enter}}{}{{enter}}t", starting_sku, ending_sku),
        SHORT_WAIT_MS / 2,
    )?;
    Ok(())
}
