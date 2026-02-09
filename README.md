# Vizualizer

Vizualizer is a Tableau-style desktop analytics tool built in Rust. It focuses on fast, in-memory exploration of Excel data with an interactive, drag-and-drop inspired workflow.

## Features

- Excel connector with multi-sheet support
- Automatic schema detection (string, number, date, boolean)
- In-memory columnar storage for rapid exploration
- Rows / Columns shelves for quick visual composition
- Interactive charts: bar, line, pie, scatter, and table
- Save/load workbooks as `.viz` project files (JSON)


## Build Instructions

1. Install Rust (stable toolchain).
2. Build the desktop app:
   ```bash
   cargo build --release
   ```
3. Run the app:
   ```bash
   cargo run
   ```

## Using Vizualizer

1. Click **Open Excel** and select your own `.xlsx`/`.xls` file.
   - To use the included sample data, open the CSVs in `examples/` with Excel or LibreOffice and save them as `.xlsx` first.
2. Pick a sheet from the left panel.
3. Add a dimension to **Rows** and a measure to **Columns**.
4. Choose a chart type (Bar, Line, Pie, Scatter, Table).
5. Save project state to a `.viz` file by exporting `VizProject` (see `storage::project`).

## Example Dataset

The `examples/` folder contains two CSVs that can be opened in Excel and saved as a workbook with two sheets:

- **Sales**: region, sales, category, and date data (`sample_sales.csv`)
- **Inventory**: items, stock levels, reorder flags (`sample_inventory.csv`)

## Documentation

- Architecture: `docs/architecture.md`
- Demo plan: `docs/demo.md`

## Notes

Vizualizer runs offline and is designed to handle files up to ~200k rows in memory. For performance, keep aggregations tight and prefer numeric measures when possible.
