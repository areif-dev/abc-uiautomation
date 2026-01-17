use uiautomation::UIAutomation;

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
    generate_simple_report_with_skips(abc_window, menu, report, 0, starting_at, ending_with)
}

/// Control ABC Client4 to generate any simple report that follows the pattern:
/// * Open F10
/// * Enter menu option
/// * Enter report option
/// * Press "Enter" to skip N secondary options
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
/// * `nskips` - The number of times "Enter" should be pressed to skip secondary options on the F10
/// screen that pop up on some reports
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
pub fn generate_simple_report_with_skips(
    abc_window: &UIElement,
    menu: &str,
    report: &str,
    nskips: u8,
    starting_at: &str,
    ending_with: &str,
) -> uiautomation::Result<()> {
    abc_window.send_keys(&format!("{{F10}}{}", menu), SHORT_WAIT_MS * 3)?;
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys(&format!("{}{{enter}}", report), SHORT_WAIT_MS / 2)?;
    for _ in 0..nskips {
        abc_window.send_keys("{enter}", SHORT_WAIT_MS / 2)?;
    }
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

/// Control ABC Client4 to generate a 7-10 report (Export Data to SQL)
///
/// # Arguments
///
/// * `abc_window` - The `UIElement` representing the Client4 window
/// * `file` - The short string representation of the ABC data file to export. Eg. the Inventory
/// file is "I", Accounts Receivable is "R"
/// * `delete_existing_data` - Whether the create tables statement should be preceded by a delete
/// statement to remove any existing data. This should probably be `false`.
/// * `starting_at` - The value to start the exported report at. If a null string is passed, then
/// the ABC provided default is used
/// * `ending_with` - The value to end the exported report at. If a null string is passed, then the
/// ABC provided default is used
pub fn generate_report_710(
    abc_window: &UIElement,
    file: &str,
    delete_existing_data: bool,
    starting_at: &str,
    ending_with: &str,
) -> uiautomation::Result<()> {
    let automation = UIAutomation::new()?;
    abc_window.send_keys("{F10}7", SHORT_WAIT_MS * 3)?;
    wait(SHORT_WAIT_MS);
    if let Ok(_) = crate::create_matcher_wrapper(&automation)?
        .classname("ThunderRT6FormDC")
        .name("Information")
        .find_first()
    {
        return Err(uiautomation::Error::new(2, &format!("ABC threw an 'Information' popup when loading the 7 report file. This probably because the user is not logged in.")));
    }
    abc_window.send_keys("10{enter}", SHORT_WAIT_MS)?;
    wait(SHORT_WAIT_MS * 3);
    abc_window.send_keys(&format!("{}{{enter}}", file), SHORT_WAIT_MS)?;
    wait(SHORT_WAIT_MS * 3);
    if delete_existing_data {
        abc_window.send_keys("y", SHORT_WAIT_MS)?;
    } else {
        abc_window.send_keys("n", SHORT_WAIT_MS)?;
    }
    wait(SHORT_WAIT_MS * 5);
    abc_window.send_keys("{enter}", SHORT_WAIT_MS)?;
    wait(SHORT_WAIT_MS);
    abc_window.send_keys(
        &format!("{}{{enter}}{}{{enter}}S", starting_at, ending_with),
        SHORT_WAIT_MS * 2,
    )?;
    Ok(())
}
