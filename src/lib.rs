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
        [self.min_line_height, self.max_line_height]
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
    fn get_best_size(&self, ratios: &[f64]) -> Result<f64, ResizeError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_fit_4_items() {
        let ratios: Vec<f64> = vec![
            (0.6678141135972461),
            (1.5086206896551724),
            (0.6666666666666666),
            (1.7396551724137932),
        ];
        let inst = ImageGrid {
            ratios,
            available_width: 1526.0,
            min_line_height: 200.0,
            max_line_height: 641.0,
            gap: 4.0,
            min_item_width: 175.0,
        };
        let result = inst.get_best_size(&inst.ratios.clone());

        assert_eq!(result, Ok(330.0));
    }

    #[test]
    fn suits_four_of_six() {
        let ratios: Vec<f64> = vec![
            (0.6678141135972461),
            (1.5086206896551724),
            (0.5623318385650224),
            (0.6666666666666666),
            (1.7396551724137932),
            (1.7396551724137932),
        ];
        let inst = ImageGrid {
            ratios,
            available_width: 1526.0,
            gap: 4.0,
            min_line_height: 200.0,
            max_line_height: 444.0,
            min_item_width: 175.0,
        };
        assert_eq!(
            inst.get_row_from_items(&mut inst.ratios.clone()),
            [(4, 444.0), (2, 437.0)]
        );
    }

    #[test]
    fn equals_must_fit_3_by_row() {
        let inst = ImageGrid {
            ratios: vec![0.875; 12],
            available_width: 1602.0,
            gap: 4.0,
            min_line_height: 200.0,
            max_line_height: 500.0,
            min_item_width: 180.0,
        };

        assert_eq!(
            inst.get_row_from_items(&mut inst.ratios.clone()),
            [(3, 500.0), (3, 500.0), (3, 500.0), (3, 500.0)]
        );
    }

    #[test]
    fn must_fit_4_squares() {
        let inst = ImageGrid {
            ratios: vec![1.0; 4],
            available_width: 800.0,
            gap: 0.0,
            min_line_height: 200.0,
            max_line_height: 500.0,
            min_item_width: 180.0,
        };

        assert_eq!(
            inst.get_row_from_items(&mut inst.ratios.clone()),
            [(4, 200.0)]
        );
    }

    #[test]
    fn fit_5th_square_next_line() {
        let inst = ImageGrid {
            ratios: vec![1.0; 5],
            available_width: 800.0,
            gap: 0.0,
            min_line_height: 200.0,
            max_line_height: 500.0,
            min_item_width: 180.0,
        };

        assert_eq!(
            inst.get_row_from_items(&mut inst.ratios.clone()),
            [(4, 200.0), (1, 500.0)]
        );
    }

    #[test]
    #[should_panic(expected = "Min height can not be bigger than max height")]
    fn test_new_min_height_bigger_than_max_height() {
        let ratios = vec![1.0, 2.0, 3.0];
        let available_width = 1000.0;
        let min_line_height = 200.0;
        let max_line_height = 100.0;
        let min_item_width = 50.0;
        let gap = 10.0;

        ImageGrid::new(
            ratios,
            available_width,
            min_line_height,
            max_line_height,
            min_item_width,
            gap,
        );
    }

    #[test]
    #[should_panic(expected = "Available width can not be less than min item width")]
    fn test_new_available_width_less_than_min_item_width() {
        let ratios = vec![1.0, 2.0, 3.0];
        let available_width = 40.0;
        let min_line_height = 100.0;
        let max_line_height = 200.0;
        let min_item_width = 50.0;
        let gap = 10.0;

        ImageGrid::new(
            ratios,
            available_width,
            min_line_height,
            max_line_height,
            min_item_width,
            gap,
        );
    }
}
