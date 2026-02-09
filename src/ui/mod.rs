use std::path::PathBuf;

use eframe::egui;
use egui::{Color32, RichText};
use egui_plot::{Bar, BarChart, Legend, Line, Plot, PlotPoints, Points};

use crate::connectors::excel;
use crate::core::workbook::Workbook;
use crate::query::engine::{run_query, QueryResult};
use crate::query::model::QuerySpec;
use crate::storage::project::{VizProject, load_project, save_project};
use crate::viz::ChartType;

pub struct VizualizerApp {
    project: VizProject,
    workbook: Option<Workbook>,
    load_error: Option<String>,
}

impl VizualizerApp {
    pub fn new() -> Self {
        Self {
            project: VizProject::new(),
            workbook: None,
            load_error: None,
        }
    }

    pub fn open_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Excel", &["xlsx", "xls"])
            .pick_file()
        {
            self.load_workbook(path, true);
        }
    }

    fn save_project_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Vizualizer Project", &["viz"])
            .set_file_name("project.viz")
            .save_file()
        {
            if let Err(error) = save_project(&path, &self.project) {
                self.load_error = Some(error.to_string());
            }
        }
    }

    fn load_project_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Vizualizer Project", &["viz"])
            .pick_file()
        {
            match load_project(&path) {
                Ok(project) => {
                    self.project = project;
                    if let Some(workbook_path) = self.project.workbook_path.clone() {
                        self.load_workbook(PathBuf::from(workbook_path), false);
                    }
                    self.load_error = None;
                }
                Err(error) => {
                    self.load_error = Some(error.to_string());
                }
            }
        }
    }

    fn load_workbook(&mut self, path: PathBuf, reset_state: bool) {
        match excel::load_workbook(&path) {
            Ok(workbook) => {
                self.project.workbook_path = Some(path.display().to_string());
                if reset_state {
                    self.project.sheet = workbook.sheets.first().map(|sheet| sheet.name.clone());
                    self.project.query = QuerySpec::empty();
                }
                self.workbook = Some(workbook);
                self.load_error = None;
            }
            Err(error) => {
                self.load_error = Some(error.to_string());
            }
        }
    }

    fn active_sheet(&self) -> Option<&crate::core::workbook::Worksheet> {
        let workbook = self.workbook.as_ref()?;
        let sheet_name = self.project.sheet.as_ref()?;
        workbook.sheets.iter().find(|sheet| &sheet.name == sheet_name)
    }

    fn query_result(&self) -> QueryResult {
        if let Some(sheet) = self.active_sheet() {
            run_query(sheet, &self.project.query)
        } else {
            QueryResult {
                headers: Vec::new(),
                rows: Vec::new(),
            }
        }
    }

    fn render_chart(&self, ui: &mut egui::Ui, result: &QueryResult) {
        match self.project.chart_type {
            ChartType::Table => render_table(ui, result),
            ChartType::Bar => render_bar(ui, result),
            ChartType::Line => render_line(ui, result),
            ChartType::Pie => render_pie(ui, result),
            ChartType::Scatter => render_scatter(ui, result),
        }
    }
}

impl eframe::App for VizualizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top-bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open Excel").clicked() {
                    self.open_dialog();
                }
                if ui.button("Load Project").clicked() {
                    self.load_project_dialog();
                }
                if ui.button("Save Project").clicked() {
                    self.save_project_dialog();
                }
                if let Some(path) = &self.project.workbook_path {
                    ui.label(format!("Workbook: {path}"));
                }
            });
        });

        egui::SidePanel::left("fields-panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Fields");

                if let Some(workbook) = &self.workbook {
                    if workbook.sheets.is_empty() {
                        ui.label("Workbook has no sheets.");
                        return;
                    }

                    ui.label("Sheet");
                    egui::ComboBox::from_id_source("sheet-selector")
                        .selected_text(self.project.sheet.clone().unwrap_or_default())
                        .show_ui(ui, |ui| {
                            for sheet in &workbook.sheets {
                                ui.selectable_value(
                                    &mut self.project.sheet,
                                    Some(sheet.name.clone()),
                                    &sheet.name,
                                );
                            }
                        });

                    let sheet = self.active_sheet().unwrap_or(&workbook.sheets[0]);
                    let column_names = sheet
                        .columns
                        .iter()
                        .map(|column| column.name.clone())
                        .collect::<Vec<_>>();

                    ui.separator();
                    ui.label(RichText::new("Dimensions").strong());
                    for column_name in column_names {
                        ui.horizontal(|ui| {
                            ui.label(&column_name);
                            if ui.small_button("Rows").clicked() {
                                add_unique(&mut self.project.query.rows, &column_name);
                            }
                            if ui.small_button("Columns").clicked() {
                                add_unique(&mut self.project.query.columns, &column_name);
                            }
                        });
                    }
                } else {
                    ui.label("Open a workbook to inspect fields.");
                }
            });

        egui::SidePanel::right("config-panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Configuration");
                ui.separator();

                ui.label("Rows shelf");
                shelf_editor(ui, &mut self.project.query.rows);

                ui.label("Columns shelf");
                shelf_editor(ui, &mut self.project.query.columns);

                ui.separator();
                ui.label("Chart type");
                for chart in ChartType::ALL {
                    ui.radio_value(&mut self.project.chart_type, chart, chart.label());
                }

                if let Some(error) = &self.load_error {
                    ui.separator();
                    ui.colored_label(Color32::RED, error);
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Visualization");
            let result = self.query_result();
            self.render_chart(ui, &result);
        });
    }
}

fn add_unique(list: &mut Vec<String>, value: &str) {
    if !list.iter().any(|item| item == value) {
        list.push(value.to_string());
    }
}

fn shelf_editor(ui: &mut egui::Ui, items: &mut Vec<String>) {
    let mut remove_index = None;
    for (index, item) in items.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.label(item);
            if ui.small_button("Remove").clicked() {
                remove_index = Some(index);
            }
        });
    }
    if let Some(index) = remove_index {
        items.remove(index);
    }
}

fn render_table(ui: &mut egui::Ui, result: &QueryResult) {
    if result.headers.is_empty() {
        ui.label("No data loaded.");
        return;
    }

    egui::ScrollArea::both().show(ui, |ui| {
        egui::Grid::new("table-grid").striped(true).show(ui, |ui| {
            for header in &result.headers {
                ui.label(RichText::new(header).strong());
            }
            ui.end_row();

            for row in &result.rows {
                for cell in row {
                    ui.label(cell);
                }
                ui.end_row();
            }
        });
    });
}

fn render_bar(ui: &mut egui::Ui, result: &QueryResult) {
    if result.rows.is_empty() || result.headers.len() < 2 {
        ui.label("Add a dimension in Rows and a measure in Columns.");
        return;
    }

    let bars = result
        .rows
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| row.get(1).and_then(|value| value.parse::<f64>().ok()).map(|y| {
            Bar::new(idx as f64, y).name(row[0].clone())
        }))
        .collect::<Vec<_>>();

    Plot::new("bar-chart")
        .legend(Legend::default())
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(BarChart::new(bars));
        });
}

fn render_line(ui: &mut egui::Ui, result: &QueryResult) {
    if result.rows.is_empty() || result.headers.len() < 2 {
        ui.label("Add a dimension in Rows and a measure in Columns.");
        return;
    }

    let points = result
        .rows
        .iter()
        .enumerate()
        .filter_map(|(idx, row)| row.get(1).and_then(|value| value.parse::<f64>().ok()).map(|y| {
            [idx as f64, y]
        }))
        .collect::<Vec<_>>();

    Plot::new("line-chart").show(ui, |plot_ui| {
        plot_ui.line(Line::new(PlotPoints::from_iter(points)));
    });
}

fn render_pie(ui: &mut egui::Ui, result: &QueryResult) {
    if result.rows.is_empty() || result.headers.len() < 2 {
        ui.label("Add a dimension in Rows and a measure in Columns.");
        return;
    }

    ui.vertical(|ui| {
        ui.label("Pie chart preview (values listed):");
        for row in &result.rows {
            if let Some(value) = row.get(1) {
                ui.label(format!("{}: {}", row.get(0).unwrap_or(&"".to_string()), value));
            }
        }
    });
}

fn render_scatter(ui: &mut egui::Ui, result: &QueryResult) {
    if result.rows.is_empty() || result.headers.len() < 3 {
        ui.label("Add a dimension in Rows and two measures in Columns.");
        return;
    }

    let points = result
        .rows
        .iter()
        .filter_map(|row| {
            let x = row.get(1)?.parse::<f64>().ok()?;
            let y = row.get(2)?.parse::<f64>().ok()?;
            Some([x, y])
        })
        .collect::<Vec<_>>();

    Plot::new("scatter-plot").show(ui, |plot_ui| {
        plot_ui.points(Points::new(PlotPoints::from_iter(points)));
    });
}
