use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use chrono::{DateTime, Utc};

/// Python wrapper for chatpack::Message
#[pyclass]
#[derive(Clone)]
pub struct PyMessage {
    #[pyo3(get, set)]
    pub sender: String,
    
    #[pyo3(get, set)]
    pub content: String,
    
    #[pyo3(get, set)]
    pub timestamp: Option<String>,
    
    #[pyo3(get, set)]
    pub platform: Option<String>,
}

#[pymethods]
impl PyMessage {
    #[new]
    #[pyo3(signature = (sender, content, timestamp=None, platform=None))]
    fn new(
        sender: String,
        content: String,
        timestamp: Option<String>,
        platform: Option<String>,
    ) -> Self {
        PyMessage {
            sender,
            content,
            timestamp,
            platform,
        }
    }
    
    fn __repr__(&self) -> String {
        format!(
            "Message(sender='{}', content='{}...', timestamp={})",
            self.sender,
            &self.content.chars().take(30).collect::<String>(),
            self.timestamp.as_deref().unwrap_or("None")
        )
    }
    
    fn __str__(&self) -> String {
        format!("{}: {}", self.sender, self.content)
    }
    
    /// Convert to dictionary
    pub fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);
        dict.set_item("sender", &self.sender)?;
        dict.set_item("content", &self.content)?;
        dict.set_item("timestamp", &self.timestamp)?;
        dict.set_item("platform", &self.platform)?;
        Ok(dict.into())
    }
}

impl PyMessage {
    pub fn from_rust(msg: chatpack::Message) -> Self {
        PyMessage {
            sender: msg.sender,
            content: msg.content,
            timestamp: msg.timestamp.map(|ts| ts.to_rfc3339()),
            platform: None, // chatpack::Message does not have platform field
        }
    }
    
    pub fn into_rust(self) -> chatpack::Message {
        chatpack::Message {
            sender: self.sender,
            content: self.content,
            timestamp: self.timestamp.and_then(|s| s.parse::<DateTime<Utc>>().ok()),
            // platform: self.platform, // chatpack::Message does not have platform field
            id: None,
            reply_to: None,
            edited: None,
        }
    }
}

/// Filter configuration for messages
#[pyclass]
#[derive(Clone)]
pub struct PyFilterConfig {
    #[pyo3(get, set)]
    pub min_length: Option<usize>,
    
    #[pyo3(get, set)]
    pub max_length: Option<usize>,
    
    #[pyo3(get, set)]
    pub sender: Option<String>,
    
    #[pyo3(get, set)]
    pub date_from: Option<String>,
    
    #[pyo3(get, set)]
    pub date_to: Option<String>,
}

#[pymethods]
impl PyFilterConfig {
    #[new]
    #[pyo3(signature = (min_length=None, max_length=None, sender=None, date_from=None, date_to=None))]
    fn new(
        min_length: Option<usize>,
        max_length: Option<usize>,
        sender: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
    ) -> Self {
        PyFilterConfig {
            min_length,
            max_length,
            sender,
            date_from,
            date_to,
        }
    }
    
    /// Set minimum message length
    fn with_min_length(mut slf: PyRefMut<'_, Self>, length: usize) -> PyRefMut<'_, Self> {
        slf.min_length = Some(length);
        slf
    }
    
    /// Set maximum message length
    fn with_max_length(mut slf: PyRefMut<'_, Self>, length: usize) -> PyRefMut<'_, Self> {
        slf.max_length = Some(length);
        slf
    }
    
    /// Filter by sender name
    fn with_sender(mut slf: PyRefMut<'_, Self>, sender: String) -> PyRefMut<'_, Self> {
        slf.sender = Some(sender);
        slf
    }
    
    /// Filter messages from this date
    fn with_date_from(mut slf: PyRefMut<'_, Self>, date: String) -> PyRefMut<'_, Self> {
        slf.date_from = Some(date);
        slf
    }
    
    /// Filter messages until this date
    fn with_date_to(mut slf: PyRefMut<'_, Self>, date: String) -> PyRefMut<'_, Self> {
        slf.date_to = Some(date);
        slf
    }
}

impl PyFilterConfig {
    pub fn into_rust(self) -> chatpack::core::filter::FilterConfig {
        let mut config = chatpack::core::filter::FilterConfig::new();
        
        // TEMPORARY FIX: methods missing
        /*
        if let Some(len) = self.min_length {
            config = config.with_min_length(len);
        }
        
        if let Some(len) = self.max_length {
            config = config.with_max_length(len);
        }
        
        if let Some(sender) = self.sender {
            config = config.with_sender(&sender);
        }
        
        if let Some(date) = self.date_from {
            if let Ok(cfg) = config.with_date_from(&date) {
                config = cfg;
            }
        }
        
        if let Some(date) = self.date_to {
            if let Ok(cfg) = config.with_date_to(&date) {
                config = cfg;
            }
        }
        */
        
        config
    }
}

/// Output configuration for exports
#[pyclass]
#[derive(Clone)]
pub struct PyOutputConfig {
    #[pyo3(get, set)]
    pub include_timestamps: bool,
    
    #[pyo3(get, set)]
    pub include_platform: bool,
}

#[pymethods]
impl PyOutputConfig {
    #[new]
    #[pyo3(signature = (include_timestamps=true, include_platform=false))]
    fn new(include_timestamps: bool, include_platform: bool) -> Self {
        PyOutputConfig {
            include_timestamps,
            include_platform,
        }
    }
    
    /// Include timestamps in output
    fn with_timestamps(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.include_timestamps = true;
        slf
    }
    
    /// Include platform information in output
    fn with_platform(mut slf: PyRefMut<'_, Self>) -> PyRefMut<'_, Self> {
        slf.include_platform = true;
        slf
    }
}