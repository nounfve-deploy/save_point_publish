#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Description {
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub title: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub label: Vec<DescLabel>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub content: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DescLabel {
    pub tag: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub val: String,
}

impl Description {
    pub fn is_empty(&self) -> bool {
        self.title.is_empty() && self.content.is_empty() && self.label.is_empty()
    }
}
