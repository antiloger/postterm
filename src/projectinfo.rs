pub struct ProjectData {
    pub name: String,
    pub id: u32,
    pub filepath: String,
    pub history: Vec<UrlHistory>,
    pub template: Vec<Template>,
    pub envvar: Vec<(String, String)>,
    pub logpath: String,
    pub meta: MetaData,
}

pub struct MetaData {}

pub struct UrlHistory {}

pub struct Template {}
