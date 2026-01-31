use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Csv,
    Json,
    Xml,
    Xls,
    Xlsx,
    Ods,
}

impl FileFormat {
    pub fn from_path(path: &Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_lowercase();

        match ext.as_str() {
            "csv" => Some(Self::Csv),
            "json" => Some(Self::Json),
            "xml" => Some(Self::Xml),
            "xls" => Some(Self::Xls),
            "xlsx" => Some(Self::Xlsx),
            "ods" => Some(Self::Ods),
            _ => None,
        }
    }

    pub fn supported_extensions() -> &'static [&'static str] {
        &["csv", "json", "xml", "xls", "xlsx", "ods"]
    }

    pub fn is_supported(path: &Path) -> bool {
        Self::from_path(path).is_some()
    }
}
