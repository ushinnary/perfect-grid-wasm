# Perfect Grid

Perfect Grid is a package that calculates image dimensions to fill a grid line while preserving the aspect ratio of all images.ing aspect ratio of all images.

## Installation

You can install Perfect Grid using either yarn or npm. Here are the commands:

```bash
yarn add perfect-grid
```

```bash
npm i perfect-grid
```

If you're using Vite, you'll need to install the `vite-plugin-wasm`

### Usage

To use Perfect Grid, you'll need to provide the following information:

- An array of aspect ratios for your images/videos/other media (in Float64Array format) `Float64Array([...items])`
- The width of the grid area `Ex: 1526`
- The minimum height for all items on a line `Ex: 200`
- The maximum height for all items on a line `Ex: 444`
- The minimum width for each item `Ex: 175`
- The gap size `Ex: 4`

```js
import { get_optimal_grid } from "perfect-grid";
const result = get_optimal_grid(
  new Float64Array([0.6678, 1.5086, 0.5623, 0.6666, 1.7396, 1.7396]), // Aspect ratios
  1526.0, // Full area width
  200.0, // Min item height
  444.0, // Max item height
  175.0, // Min item width
  4.0 // Gap
);

// The result is a list of height per item
console.log(result); // [444, 444, 444, 444, 437, 437]
```

To use Perfect Grid with ResizeObserver, you can create a new instance of ResizeObserver and attach it to the container element that you want to observe for size changes. Then, in the callback function for the ResizeObserver, you can call the get_optimal_grid function with the updated container width and the other required parameters.

Here's an example implementation:

```js
import { get_optimal_grid } from "perfect-grid";

// Get the container element that you want to observe for size changes
const container = document.getElementById("my-container");

// Create a new instance of ResizeObserver and attach it to the container element
const resizeObserver = new ResizeObserver((entries) => {
  // Get the new container width from the first entry in the list of entries
  const newWidth = entries[0].contentRect.width;

  // Call the get_optimal_grid function with the updated container width and the other required parameters
  const result = get_optimal_grid(
    new Float64Array([0.6678, 1.5086, 0.5623, 0.6666, 1.7396, 1.7396]), // Aspect ratios
    newWidth, // Full area width
    200.0, // Min item height
    444.0, // Max item height
    175.0, // Min item width
    4.0 // Gap
  );

  // Update the item heights with the new values from the result
  const items = container.querySelectorAll(".item");
  items.forEach((item, index) => {
    item.style.height = `${result[index]}px`;
  });
});

// Start observing the container element for size changes
resizeObserver.observe(container);
```
