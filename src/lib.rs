pub mod accounts_receivable;
pub mod customer_file;
pub mod reports;

use std::{thread, time};
use uiautomation::{UIAutomation, UIMatcher, UITreeWalker};

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

/// Attempt to read the value of any RT6ThunderTextBox on a given ABC Client4 screen
///
/// # Arguments
///
/// * `screen` - The `UIElement` which represents the Client4 screen to look for textboxes on
/// * `box_index` - The index (starting from 0) of the RT6ThunderTextBox to read
///
/// # Returns
///
/// If the textbox is found successfully, return the string value of that textbox. If the value is
/// empty, return `String::new()`
///
/// # Errors
///
/// Return `Err(uiautomation::Error)` if the textbox cannot be found or its value fails to convert
/// to string for some reason
pub fn read_text_box_value(screen: &UIElement, box_index: usize) -> uiautomation::Result<String> {
    let automation = UIAutomation::new()?;

    let all_text_boxes = create_matcher_wrapper(&automation)?
        .from(screen.to_owned())
        .classname("ThunderRT6TextBox")
        .find_all()?;

    let desired_txtbx = match all_text_boxes.get(box_index) {
        Some(b) => b,
        None => {
            return Err(uiautomation::Error::new(
                2,
                &format!("No textbox found at index {}", box_index),
            ))
        }
    };

    Ok(desired_txtbx
        .get_property_value(uiautomation::types::UIProperty::ValueValue)?
        .get_string()?)
}
