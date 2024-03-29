#[cfg(test)]
mod tests {
    use perfect_grid::*;

    #[test]
    fn must_fit_4_items() {
        let ratios: Vec<f64> = vec![
            (0.6678141135972461),
            (1.5086206896551724),
            (0.6666666666666666),
            (1.7396551724137932),
        ];
        let inst = ImageGrid {
            available_width: 1526.0,
            min_line_height: 200.0,
            max_line_height: 641.0,
            gap: 4.0,
            min_item_width: 175.0,
        };
        let result = inst.get_optimal_height(&ratios);

        assert_eq!(result, 330.0);
    }

    #[test]
    fn suits_four_of_six() {
        let mut ratios: Vec<f64> = vec![
            (0.6678141135972461),
            (1.5086206896551724),
            (0.5623318385650224),
            (0.6666666666666666),
            (1.7396551724137932),
            (1.7396551724137932),
        ];
        let inst = ImageGrid {
            available_width: 1526.0,
            gap: 4.0,
            min_line_height: 200.0,
            max_line_height: 444.0,
            min_item_width: 175.0,
        };
        assert_eq!(
            inst.get_row_from_items(&mut ratios),
            [(4, 444.0), (2, 437.0)]
        );
    }

    #[test]
    fn equals_must_fit_3_by_row() {
        let mut ratios = vec![0.875; 12];
        let inst = ImageGrid {
            available_width: 1602.0,
            gap: 4.0,
            min_line_height: 200.0,
            max_line_height: 500.0,
            min_item_width: 180.0,
        };

        assert_eq!(
            inst.get_row_from_items(&mut ratios),
            [(8, 224.0), (4, 454.0)]
        );
    }

    #[test]
    fn must_fit_4_squares() {
        let mut ratios = vec![1.0; 4];
        let inst = ImageGrid {
            available_width: 800.0,
            gap: 0.0,
            min_line_height: 200.0,
            max_line_height: 500.0,
            min_item_width: 180.0,
        };

        assert_eq!(inst.get_row_from_items(&mut ratios), [(4, 200.0)]);
    }

    #[test]
    fn fit_5th_square_next_line() {
        let mut ratios = vec![1.0; 5];
        let inst = ImageGrid {
            available_width: 800.0,
            gap: 0.0,
            min_line_height: 200.0,
            max_line_height: 500.0,
            min_item_width: 180.0,
        };

        assert_eq!(
            inst.get_row_from_items(&mut ratios),
            [(4, 200.0), (1, 500.0)]
        );
    }

    #[test]
    #[should_panic(expected = "Min height can not be bigger than max height")]
    fn test_new_min_height_bigger_than_max_height() {
        let available_width = 1000.0;
        let min_line_height = 200.0;
        let max_line_height = 100.0;
        let min_item_width = 50.0;
        let gap = 10.0;

        ImageGrid::new(
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
        let available_width = 40.0;
        let min_line_height = 100.0;
        let max_line_height = 200.0;
        let min_item_width = 50.0;
        let gap = 10.0;

        ImageGrid::new(
            available_width,
            min_line_height,
            max_line_height,
            min_item_width,
            gap,
        );
    }

    #[test]
    fn test_20_items_real_example() {
        let mut ratios = vec![
            0.875,
            0.875,
            0.875,
            16.0 / 9.0,
            3.5555555555555554,
            0.875,
            0.875,
            0.875,
            0.6648401826484018,
            0.875,
            16.0 / 9.0,
            0.875,
            16.0 / 9.0,
            16.0 / 9.0,
            16.0 / 9.0,
            0.875,
            0.875,
            0.875,
            0.875,
            0.875,
        ];
        let inst = ImageGrid {
            available_width: 1526.0,
            gap: 4.0,
            max_line_height: 575.0,
            min_item_width: 175.0,
            min_line_height: 200.0,
        };

        assert_eq!(
            inst.get_row_from_items(&mut ratios),
            [(4, 343.0), (4, 244.0), (4, 361.0), (5, 213.0), (3, 575.0),]
        );
    }
}
