use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq)]
pub enum ResizeError {
    MinItemWidthOverload,
    LowerThanMinHeight,
    BiggerThanMaxHeight,
    CanNotFitItems,
    Empty,
}

pub struct ImageGrid {
    pub available_width: f64,
    pub min_line_height: f64,
    pub max_line_height: f64,
    pub min_item_width: f64,
    pub gap: f64,
    pub items: Vec<f64>,
}

impl ImageGrid {
    pub fn new(
        items: Vec<f64>,
        available_width: f64,
        min_line_height: f64,
        max_line_height: f64,
        min_item_width: f64,
        gap: f64,
    ) -> Self {
        assert!(min_line_height > max_line_height);
        assert!(available_width < min_item_width);

        ImageGrid {
            items,
            available_width,
            min_line_height,
            max_line_height,
            min_item_width,
            gap,
        }
    }

    fn calculate_all_width_by_height(&self, items: Vec<f64>, desired_height: f64) -> f64 {
        let sum: f64 = items
            .into_iter()
            .map(|item| desired_height * item + self.gap)
            .sum();
        sum - self.gap
    }

    fn calculate_all_width_by_height_secure(
        &self,
        items: Vec<f64>,
        desired_height: f64,
    ) -> Result<f64, ResizeError> {
        if items
            .clone()
            .into_iter()
            .any(|item| (desired_height * item) + self.gap < self.min_item_width)
        {
            return Err(ResizeError::MinItemWidthOverload);
        }

        let width = self.calculate_all_width_by_height(items, desired_height);

        if width > self.available_width {
            return Err(ResizeError::CanNotFitItems);
        }

        Ok(width)
    }

    /// In case if it's OK will return space left
    fn may_fit_in_width(&self, items: Vec<f64>, height: f64) -> Result<f64, ResizeError> {
        if height < self.min_line_height {
            return Err(ResizeError::LowerThanMinHeight);
        } else if height > self.max_line_height {
            return Err(ResizeError::BiggerThanMaxHeight);
        }

        let all_width = self.calculate_all_width_by_height_secure(items, height);

        if let Ok(all_width) = all_width {
            if all_width <= self.available_width {
                return Ok(self.available_width - all_width);
            }
        }

        Err(ResizeError::CanNotFitItems)
    }

    fn items_may_be_fitted(&self, items: Vec<f64>) -> Result<bool, ResizeError> {
        let max_height_fit =
            self.calculate_all_width_by_height_secure(items.clone(), self.max_line_height);
        let min_height_fit = self.calculate_all_width_by_height_secure(items, self.min_line_height);

        if max_height_fit.is_ok() || min_height_fit.is_ok() {
            Ok(true)
        } else {
            Err(ResizeError::CanNotFitItems)
        }
    }

    fn get_js_array_from_values(&self, first: u32, last: f64) -> js_sys::Array {
        let arr = js_sys::Array::new();
        arr.set(0, JsValue::from(first));
        arr.set(1, JsValue::from_f64(last));

        arr
    }

    /// Returns vector of tuples with number of items to take and height for them
    pub fn get_row_from_items(&self, items: Vec<f64>) -> Vec<(u32, f64)> {
        if items.is_empty() {
            return vec![(0, 0.0)];
        }

        let mut copy = items;
        let mut not_fitted: Vec<f64> = Vec::new();

        while self.items_may_be_fitted((*copy).to_vec()).is_err() {
            if copy.is_empty() {
                break;
            }

            if copy.len() == 1 {
                return vec![(1, self.min_line_height)];
            }

            not_fitted.insert(0, copy.pop().unwrap());
        }

        if copy.is_empty() {
            return vec![(0, 0.0)];
        }

        let best_size_for_suitable = self.get_best_size(copy.clone());
        let best_height: f64 = match best_size_for_suitable {
            Err(_) => {
                if copy.is_empty() {
                    return vec![(0, 0.0)];
                }

                if copy.len() == 1 {
                    return vec![(1, self.min_line_height)];
                }

                return self.get_row_from_items(copy);
            }
            Ok(res) => res,
        };

        let mut result = vec![(copy.len() as u32, best_height)];

        if !not_fitted.is_empty() {
            let best_size_for_unfitted = self.get_best_size(not_fitted.clone());
            if let Ok(bs) = best_size_for_unfitted {
                result.push((not_fitted.len() as u32, bs));
            } else {
                let rest_filtered = &mut self.get_row_from_items(not_fitted);
                result.append(rest_filtered);
                return result;
            }
        }

        result
    }

    fn get_average_width_by_px(&self, items: Vec<f64>) -> f64 {
        let first = self.calculate_all_width_by_height(items.clone(), 0.0);
        let second = self.calculate_all_width_by_height(items, 1.0);

        second - first
    }

    /// Returns best height for items:
    fn get_best_size(&self, items: Vec<f64>) -> Result<f64, ResizeError> {
        if items.is_empty() {
            return Err(ResizeError::Empty);
        }

        let mut fitted_already = false;
        let mut height_and_remaining_space = (0.0, 0.0);
        let avg_by_height = self.get_average_width_by_px(items.clone());
        // Will always be slightly bigger
        let mut height = (self.available_width / avg_by_height).floor();

        if height > self.max_line_height {
            height = self.max_line_height;
        }

        while height >= self.min_line_height && height <= self.max_line_height {
            if let Ok(res) = self.may_fit_in_width(items.clone(), height) {
                if fitted_already && res > height_and_remaining_space.1 {
                    break;
                }

                height_and_remaining_space.0 = height;
                height_and_remaining_space.1 = res;
                fitted_already = true;
            } else if fitted_already {
                return Ok(height_and_remaining_space.0);
            }

            height -= 1.0;
        }

        if !fitted_already {
            return Err(ResizeError::CanNotFitItems);
        }

        Ok(height_and_remaining_space.0)
    }
}

#[wasm_bindgen]
pub fn get_optimal_grid(
    items: Vec<f64>,
    available_width: f64,
    min_line_height: f64,
    max_line_height: f64,
    min_item_width: f64,
    gap: f64,
) -> js_sys::Array {
    let inst = ImageGrid::new(
        items,
        available_width,
        min_line_height,
        max_line_height,
        min_item_width,
        gap,
    );

    let result_array = js_sys::Array::new();

    let res = inst.get_row_from_items(inst.items.clone());
    for (i, item) in res.iter().enumerate() {
        result_array.set(
            i as u32,
            JsValue::from(inst.get_js_array_from_values(item.0, item.1)),
        );
    }
    result_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_fit_4_items() {
        // let available_width = 1526.0;
        let items: Vec<f64> = vec![
            (0.6678141135972461),
            (1.5086206896551724),
            (0.6666666666666666),
            (1.7396551724137932),
        ];
        let inst = ImageGrid {
            items,
            available_width: 1526.0,
            min_line_height: 200.0,
            max_line_height: 641.0,
            gap: 4.0,
            min_item_width: 175.0,
        };
        let result = inst.get_best_size(inst.items.clone());

        assert_eq!(result, Ok(330.0));
    }

    #[test]
    fn suits_four_of_six() {
        let items: Vec<f64> = vec![
            (0.6678141135972461),
            (1.5086206896551724),
            (0.5623318385650224),
            (0.6666666666666666),
            (1.7396551724137932),
            (1.7396551724137932),
        ];
        let inst = ImageGrid {
            items,
            available_width: 1526.0,
            gap: 4.0,
            min_line_height: 200.0,
            max_line_height: 444.0,
            min_item_width: 175.0,
        };
        assert_eq!(
            inst.get_row_from_items(inst.items.clone()),
            [(4, 444.0), (2, 437.0)]
        );
    }
}
