use serde::{Deserialize, Serialize};

use crate::query::model::QuerySpec;
use crate::viz::ChartType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VizProject {
    pub workbook_path: Option<String>,
    pub sheet: Option<String>,
    pub query: QuerySpec,
    pub chart_type: ChartType,
}

impl VizProject {
    pub fn new() -> Self {
        Self {
            workbook_path: None,
            sheet: None,
            query: QuerySpec::empty(),
            chart_type: ChartType::Table,
        }
    }
}

pub fn save_project(path: &std::path::Path, project: &VizProject) -> anyhow::Result<()> {
    let payload = serde_json::to_vec_pretty(project)?;
    std::fs::write(path, payload)?;
    Ok(())
}

pub fn load_project(path: &std::path::Path) -> anyhow::Result<VizProject> {
    let payload = std::fs::read(path)?;
    Ok(serde_json::from_slice(&payload)?)
}
