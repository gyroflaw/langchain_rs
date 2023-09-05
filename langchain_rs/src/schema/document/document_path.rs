use anyhow::Result;

#[derive(Debug)]
pub enum DocumentPath {
    Str(String),
    List(Vec<usize>),
}

impl DocumentPath {
    pub fn as_str(&self) -> Result<&str> {
        match self {
            DocumentPath::Str(s) => Ok(s),
            _ => {
                anyhow::bail!("DocumentPath is not a string, {:?}", self)
            }
        }
    }

    pub fn from_string(s: &str) -> Self {
        DocumentPath::Str(s.to_string())
    }

    pub fn as_list(&self) -> Result<Vec<usize>> {
        match self {
            DocumentPath::List(list) => Ok(list.clone()),
            _ => {
                anyhow::bail!("DocumentPath is not a list, {:?}", self)
            }
        }
    }

    pub fn from_list(list: Vec<usize>) -> Self {
        DocumentPath::List(list)
    }
}
