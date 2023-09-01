use wasm_bindgen::prelude::*;

// Custom error for better handling
#[derive(Debug, PartialEq, Eq)]
pub enum ResizeError {
    MinItemWidthOverload,
    LowerThanMinHeight,
    BiggerThanMaxHeight,
    CanNotFitItems,
    Empty,
}

// Main struct with essentials props
#[derive(Debug)]
pub struct ImageGrid {
    pub available_width: f64,
    pub min_line_height: f64,
    pub max_line_height: f64,
    pub min_item_width: f64,
    pub gap: f64,
}

impl ImageGrid {
    pub fn new(
        available_width: f64,
        min_line_height: f64,
        max_line_height: f64,
        min_item_width: f64,
        gap: f64,
    ) -> Self {
        assert!(
            min_line_height <= max_line_height,
            "Min height can not be bigger than max height"
        );
        assert!(
            available_width >= min_item_width,
            "Available width can not be less than min item width"
        );

        ImageGrid {
            available_width,
            min_line_height,
            max_line_height,
            min_item_width,
            gap,
        }
    }

    // Getting sum of all elements after multiply
    fn calculate_all_width_by_height(&self, ratios: &[f64], desired_height: f64) -> f64 {
        ratios.iter().fold(-self.gap, |acc, &ratio| {
            acc + desired_height * ratio + self.gap
        })
    }

    /// The calculate_all_width_by_height_secure function calculates the width of all items by a given height. The function checks that the width of the items does not exceed the available width, and that the width of the items does not exceed the minimum item width.
    fn calculate_all_width_by_height_secure(
        &self,
        ratios: &[f64],
        desired_height: f64,
    ) -> Result<f64, ResizeError> {
        if ratios
            .iter()
            .any(|ratio| (desired_height * ratio) + self.gap < self.min_item_width)
        {
            return Err(ResizeError::MinItemWidthOverload);
        }

        let width = self.calculate_all_width_by_height(ratios, desired_height);

        if width > self.available_width {
            Err(ResizeError::CanNotFitItems)
        } else {
            Ok(width)
        }
    }

    /// Check if we may fit all items with max or min line height
    fn items_may_be_fitted(&self, ratios: &[f64], height: f64) -> bool {
        self.calculate_all_width_by_height_secure(ratios, height)
            .is_ok()
    }

    /// Returns vector of tuples with number of items to take and height for them
    pub fn get_row_from_items(&self, ratios: &mut Vec<f64>) -> Vec<(u32, f64)> {
        let mut not_fitted: Vec<f64> = Vec::new();
        let mut height_for_ratios = self.get_optimal_height(ratios);

        for ratio_index in 0..ratios.len() {
            let items = &ratios[0..=ratio_index];
            let new_height = self.get_optimal_height(items);
            if !self.items_may_be_fitted(items, new_height) {
                not_fitted = ratios.drain(ratio_index..).collect();
                break;
            }

            height_for_ratios = new_height.min(self.max_line_height);
        }

        let mut result = vec![(ratios.len() as u32, height_for_ratios)];

        if !not_fitted.is_empty() {
            let rest_filtered = &mut self.get_row_from_items(&mut not_fitted);
            result.append(rest_filtered);
        }

        result
    }

    pub fn get_optimal_height(&self, ratios: &[f64]) -> f64 {
        let gaps_width = ratios.len().saturating_sub(1) as f64 * self.gap;
        let total_width = self.available_width - gaps_width;
        (total_width / ratios.iter().sum::<f64>()).floor()
    }
}

#[wasm_bindgen]
pub fn get_optimal_grid(
    ratios: Vec<f64>,
    available_width: f64,
    min_line_height: f64,
    max_line_height: f64,
    min_item_width: f64,
    gap: f64,
) -> js_sys::Array {
    let mut ratios = ratios;
    let grid = ImageGrid::new(
        available_width,
        min_line_height,
        max_line_height,
        min_item_width,
        gap,
    );

    let height_list = grid.get_row_from_items(&mut ratios).into_iter();

    height_list
        .flat_map(|(count, height)| std::iter::repeat(height).take(count as usize))
        .map(JsValue::from)
        .collect::<js_sys::Array>()
}
