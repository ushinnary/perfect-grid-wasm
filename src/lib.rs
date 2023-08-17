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
    pub ratios: Vec<f64>,
    pub available_width: f64,
    pub min_line_height: f64,
    pub max_line_height: f64,
    pub min_item_width: f64,
    pub gap: f64,
}

impl ImageGrid {
    pub fn new(
        ratios: Vec<f64>,
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
            ratios,
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

    /// In case if it's OK will return space left
    fn may_fit_in_width(&self, ratios: &[f64], height: f64) -> Result<f64, ResizeError> {
        if height < self.min_line_height {
            return Err(ResizeError::LowerThanMinHeight);
        } else if height > self.max_line_height {
            return Err(ResizeError::BiggerThanMaxHeight);
        }

        let all_width = self.calculate_all_width_by_height_secure(ratios, height);

        all_width.map_or(Err(ResizeError::CanNotFitItems), |all_width| {
            if all_width <= self.available_width {
                Ok(self.available_width - all_width)
            } else {
                Err(ResizeError::CanNotFitItems)
            }
        })
    }

    /// Check if we may fit all items with max or min line height
    fn items_may_be_fitted(&self, ratios: &[f64]) -> bool {
        dbg!(&ratios, self.get_optimal_height(ratios));
        [
            self.min_line_height,
            self.get_optimal_height(ratios),
            self.max_line_height,
        ]
        .iter()
        .any(|&height| {
            self.calculate_all_width_by_height_secure(ratios, height)
                .is_ok()
        })
    }

    /// Returns vector of tuples with number of items to take and height for them
    pub fn get_row_from_items(&self, ratios: &mut Vec<f64>) -> Vec<(u32, f64)> {
        if ratios.is_empty() {
            return vec![(0, 0.0)];
        }

        let mut not_fitted: Vec<f64> = Vec::new();

        while !self.items_may_be_fitted(ratios) {
            not_fitted.insert(0, ratios.pop().unwrap());
        }

        if ratios.is_empty() {
            return vec![(0, 0.0)];
        }

        let best_size_for_suitable = self.get_best_size(ratios);
        let best_height: f64 = match best_size_for_suitable {
            Err(_) => {
                if ratios.len() == 1 {
                    return vec![(1, self.min_line_height)];
                }

                return vec![(0, 0.0)];
            }
            Ok(res) => res,
        };

        let mut result = vec![(ratios.len() as u32, best_height)];

        if !not_fitted.is_empty() {
            let rest_filtered = &mut self.get_row_from_items(&mut not_fitted);
            result.append(rest_filtered);
        }

        result
    }

    fn get_optimal_height(&self, ratios: &[f64]) -> f64 {
        let gaps_width = ratios.len().saturating_sub(1) as f64 * self.gap;
        let total_width = self.available_width - gaps_width;
        (total_width / ratios.iter().sum::<f64>()).floor()
    }

    /// Returns best height for items:
    pub fn get_best_size(&self, ratios: &[f64]) -> Result<f64, ResizeError> {
        if ratios.is_empty() {
            return Err(ResizeError::Empty);
        }

        let mut fitted_already = false;
        let mut height_and_remaining_space = (0.0, 0.0);
        let mut height = self.get_optimal_height(ratios).min(self.max_line_height);

        while let Ok(space_left) = self.may_fit_in_width(ratios, height) {
            if fitted_already && space_left > height_and_remaining_space.1 {
                break;
            }

            height_and_remaining_space = (height, space_left);
            fitted_already = true;
            height -= 1.0;
        }

        if fitted_already {
            Ok(height_and_remaining_space.0)
        } else {
            Err(ResizeError::CanNotFitItems)
        }
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
    let grid = ImageGrid::new(
        ratios,
        available_width,
        min_line_height,
        max_line_height,
        min_item_width,
        gap,
    );

    let height_list = grid
        .get_row_from_items(&mut grid.ratios.clone())
        .into_iter();

    height_list
        .flat_map(|(count, height)| std::iter::repeat(height).take(count as usize))
        .map(JsValue::from)
        .collect::<js_sys::Array>()
}
