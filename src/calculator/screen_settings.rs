pub static BUTTON_WIDTH: f32 = 57.5;
pub static BUTTON_HEIGHT: f32 = 47.5;
pub static COLUMN_SPACE: f32 = 0.5;
pub static ROW_SPACE: f32 = 0.5;
pub static ELEMENTS_IN_ROW: u16 = 4;
pub static ELEMENTS_IN_COLUMN: u16 = 5;
pub static DISPLAY_HEIGHT: f32 = 60.0;

/// Calculates the total width of the application window based on the number
/// of elements per row, the space between elements, and the width
/// of each button.
///
/// # Returns
/// Returns the total width of the application window as a `f32` value.
///
/// # Example
/// ```
/// use rustcalculator::calculator::screen_settings::get_app_width;
///
/// // Assuming a specific number of elements in each row, row space,
/// // and button width.
/// let width = get_app_width();
/// println!("Total width of the application window: {}", width);
/// ```
pub fn get_app_width() -> f32 {
    let total_space = (ELEMENTS_IN_ROW - 2) as f32 * ROW_SPACE;
    let total_items = ELEMENTS_IN_ROW as f32 * BUTTON_WIDTH;

    total_space + (total_items) as f32
}

/// Calculates the total height of the application window based on the number
/// of elements per column, the space between elements, and the height
/// of each button and the display area.
///
/// # Returns
/// Returns the total height of the application window as a `f32` value.
///
/// # Example
/// ```
/// use rustcalculator::calculator::screen_settings::get_app_height;
///
/// // Assuming a specific number of elements in each column, column space,
/// // button height, and display height.
/// let height = get_app_height();
/// println!("Total height of the application window: {}", height);
/// ```
pub fn get_app_height() -> f32 {
    let total_space = (ELEMENTS_IN_COLUMN - 1) as f32 * COLUMN_SPACE;
    let total_items = ELEMENTS_IN_COLUMN as f32 * BUTTON_HEIGHT;

    total_space + (total_items) + DISPLAY_HEIGHT as f32
}