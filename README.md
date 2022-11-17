This package calculates images dimensions to fill the grid line by saving aspect ratio of all images.

## Important!

It's a wasm package so you have to initiate it first

### Usage example

```
import init, {get_optimal_grid} from 'perfect-grid';
init().then(() => {
	const result = get_optimal_grid(new Float64Array([
		0.6678,
		1.5086,
		0.5623,
		0.6666,
		1.7396,
		1.7396,
	]), 1526.0, 200.0, 444.0, 175.0, 4.0);

	console.log({ result });
})
```
