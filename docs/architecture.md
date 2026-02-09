# Vizualizer Architecture

## Overview

Vizualizer is a modular Rust application that mirrors Tableau-style analytics workflows. The architecture is organized around a data connector, an in-memory columnar engine, a lightweight query layer, a desktop UI, and visualization rendering.

## Modules

### `connectors/`
- Reads Excel workbooks (`.xlsx`/`.xls`) via `calamine`.
- Supports multiple sheets and converts rows into columnar vectors.
- Infers data types per column.

### `core/`
- Defines `DataValue`, `DataType`, `Column`, `Worksheet`, and `Workbook`.
- Stores values in memory for rapid filtering and aggregation.

### `query/`
- Exposes `QuerySpec` for rows/columns shelves, aggregations, and filters.
- `engine::run_query` performs grouping and aggregation in-process.

### `viz/`
- Tracks chart types and visualization configuration.

### `ui/`
- Built with `eframe`/`egui` for a native desktop experience.
- Provides fields panel, shelf configuration, and chart preview.

### `storage/`
- Defines `.viz` project format as JSON (`VizProject`).

## Data Flow

1. User selects an Excel file.
2. Connector loads sheets into `Workbook`.
3. UI selects a sheet and builds a `QuerySpec` from shelves.
4. Query engine returns tabular results.
5. Visualization layer renders charts from query output.

## Performance Considerations

- Columnar storage reduces scan overhead for aggregation.
- Query engine uses in-memory hash maps for grouping.
- UI refreshes charts on each interaction for near-real-time feedback.
