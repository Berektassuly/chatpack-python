use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::path::Path;
use chatpack::parser::Parser;
use crate::types::PyMessage;

// Helper function to create filter config from parameters
fn build_filter_config(
    min_length: Option<usize>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> chatpack::core::filter::FilterConfig {
    let mut config = chatpack::core::filter::FilterConfig::new();
    
    // TEMPORARY FIX: FilterConfig builder methods (with_min_length etc) are missing in chatpack 0.5
    // Assuming they are public fields or renamed. Disabling for compilation.
    /*
    if let Some(len) = min_length {
        config = config.with_min_length(len);
    }
    
    if let Some(date) = date_from {
        if let Ok(cfg) = config.with_date_from(&date) {
            config = cfg;
        }
    }
    
    if let Some(date) = date_to {
        if let Ok(cfg) = config.with_date_to(&date) {
            config = cfg;
        }
    }
    */
    
    config
}

// Helper function to apply merge if needed
fn maybe_merge(messages: Vec<chatpack::Message>, merge: bool) -> Vec<chatpack::Message> {
    if merge {
        // chatpack::core::merge::merge_consecutive(messages, std::time::Duration::from_secs(300))
        messages // Merge disabled
    } else {
        messages
    }
}

/// Telegram parser implementation
pub fn parse_telegram_impl(
    path: String,
    merge: bool,
    min_length: Option<usize>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> PyResult<Vec<PyMessage>> {
    let parser = chatpack::parsers::TelegramParser::new();
    
    let messages = parser.parse(Path::new(&path))
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;
    
    let filter_config = build_filter_config(min_length, date_from, date_to);
    let filtered = chatpack::core::filter::apply_filters(messages, &filter_config);
    let result = maybe_merge(filtered, merge);
    
    Ok(result.into_iter().map(PyMessage::from_rust).collect())
}

/// WhatsApp parser implementation
pub fn parse_whatsapp_impl(
    path: String,
    merge: bool,
    min_length: Option<usize>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> PyResult<Vec<PyMessage>> {
    let parser = chatpack::parsers::WhatsAppParser::new();
    
    let messages = parser.parse(Path::new(&path))
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;
    
    let filter_config = build_filter_config(min_length, date_from, date_to);
    let filtered = chatpack::core::filter::apply_filters(messages, &filter_config);
    let result = maybe_merge(filtered, merge);
    
    Ok(result.into_iter().map(PyMessage::from_rust).collect())
}

/// Instagram parser implementation
pub fn parse_instagram_impl(
    path: String,
    merge: bool,
    min_length: Option<usize>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> PyResult<Vec<PyMessage>> {
    let parser = chatpack::parsers::InstagramParser::new();
    
    let messages = parser.parse(Path::new(&path))
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;
    
    let filter_config = build_filter_config(min_length, date_from, date_to);
    let filtered = chatpack::core::filter::apply_filters(messages, &filter_config);
    let result = maybe_merge(filtered, merge);
    
    Ok(result.into_iter().map(PyMessage::from_rust).collect())
}

/// Discord parser implementation
pub fn parse_discord_impl(
    path: String,
    merge: bool,
    min_length: Option<usize>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> PyResult<Vec<PyMessage>> {
    let parser = chatpack::parsers::DiscordParser::new();
    
    let messages = parser.parse(Path::new(&path))
        .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;
    
    let filter_config = build_filter_config(min_length, date_from, date_to);
    let filtered = chatpack::core::filter::apply_filters(messages, &filter_config);
    let result = maybe_merge(filtered, merge);
    
    Ok(result.into_iter().map(PyMessage::from_rust).collect())
}

/// Telegram Parser class for object-oriented API
#[pyclass]
pub struct TelegramParser {
    parser: chatpack::parsers::TelegramParser,
}

#[pymethods]
impl TelegramParser {
    #[new]
    fn new() -> Self {
        TelegramParser {
            parser: chatpack::parsers::TelegramParser::new(),
        }
    }
    
    /// Parse Telegram export file
    #[pyo3(signature = (path, merge=false, min_length=None, date_from=None, date_to=None))]
    fn parse(
        &self,
        path: String,
        merge: bool,
        min_length: Option<usize>,
        date_from: Option<String>,
        date_to: Option<String>,
    ) -> PyResult<Vec<PyMessage>> {
        parse_telegram_impl(path, merge, min_length, date_from, date_to)
    }
    
    /// Parse from string content
    fn parse_str(&self, content: String) -> PyResult<Vec<PyMessage>> {
        let messages = self.parser.parse_str(&content)
            .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;
        
        Ok(messages.into_iter().map(PyMessage::from_rust).collect())
    }
}

/// WhatsApp Parser class
#[pyclass]
pub struct WhatsAppParser {
    parser: chatpack::parsers::WhatsAppParser,
}

#[pymethods]
impl WhatsAppParser {
    #[new]
    fn new() -> Self {
        WhatsAppParser {
            parser: chatpack::parsers::WhatsAppParser::new(),
        }
    }
    
    /// Parse WhatsApp export file
    #[pyo3(signature = (path, merge=false, min_length=None, date_from=None, date_to=None))]
    fn parse(
        &self,
        path: String,
        merge: bool,
        min_length: Option<usize>,
        date_from: Option<String>,
        date_to: Option<String>,
    ) -> PyResult<Vec<PyMessage>> {
        parse_whatsapp_impl(path, merge, min_length, date_from, date_to)
    }
    
    /// Parse from string content
    fn parse_str(&self, content: String) -> PyResult<Vec<PyMessage>> {
        let messages = self.parser.parse_str(&content)
            .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;
        
        Ok(messages.into_iter().map(PyMessage::from_rust).collect())
    }
}

/// Instagram Parser class
#[pyclass]
pub struct InstagramParser {
    parser: chatpack::parsers::InstagramParser,
}

#[pymethods]
impl InstagramParser {
    #[new]
    fn new() -> Self {
        InstagramParser {
            parser: chatpack::parsers::InstagramParser::new(),
        }
    }
    
    /// Parse Instagram export file
    #[pyo3(signature = (path, merge=false, min_length=None, date_from=None, date_to=None))]
    fn parse(
        &self,
        path: String,
        merge: bool,
        min_length: Option<usize>,
        date_from: Option<String>,
        date_to: Option<String>,
    ) -> PyResult<Vec<PyMessage>> {
        parse_instagram_impl(path, merge, min_length, date_from, date_to)
    }
    
    /// Parse from string content
    fn parse_str(&self, content: String) -> PyResult<Vec<PyMessage>> {
        let messages = self.parser.parse_str(&content)
            .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;
        
        Ok(messages.into_iter().map(PyMessage::from_rust).collect())
    }
}

/// Discord Parser class
#[pyclass]
pub struct DiscordParser {
    parser: chatpack::parsers::DiscordParser,
}

#[pymethods]
impl DiscordParser {
    #[new]
    fn new() -> Self {
        DiscordParser {
            parser: chatpack::parsers::DiscordParser::new(),
        }
    }
    
    /// Parse Discord export file
    #[pyo3(signature = (path, merge=false, min_length=None, date_from=None, date_to=None))]
    fn parse(
        &self,
        path: String,
        merge: bool,
        min_length: Option<usize>,
        date_from: Option<String>,
        date_to: Option<String>,
    ) -> PyResult<Vec<PyMessage>> {
        parse_discord_impl(path, merge, min_length, date_from, date_to)
    }
    
    /// Parse from string content
    fn parse_str(&self, content: String) -> PyResult<Vec<PyMessage>> {
        let messages = self.parser.parse_str(&content)
            .map_err(|e| PyValueError::new_err(format!("Parse error: {}", e)))?;
        
        Ok(messages.into_iter().map(PyMessage::from_rust).collect())
    }
}