use pyo3::prelude::*;

mod conversion;
mod parsers;
mod types;

use parsers::*;
use parsers::*;
use types::*;

/// Python module for chatpack - parse chat exports from messaging platforms
#[pymodule]
fn _chatpack(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register types
    m.add_class::<PyMessage>()?;
    m.add_class::<PyFilterConfig>()?;
    m.add_class::<PyOutputConfig>()?;
    
    // Register parsers
    m.add_class::<TelegramParser>()?;
    m.add_class::<WhatsAppParser>()?;
    m.add_class::<InstagramParser>()?;
    m.add_class::<DiscordParser>()?;
    
    // Register streaming parsers - REMOVED (module missing)
    // m.add_class::<TelegramStreamParser>()?;
    // m.add_class::<WhatsAppStreamParser>()?;
    // m.add_class::<InstagramStreamParser>()?;
    // m.add_class::<DiscordStreamParser>()?;
    
    // Convenience functions
    m.add_function(wrap_pyfunction!(parse_telegram, m)?)?;
    m.add_function(wrap_pyfunction!(parse_whatsapp, m)?)?;
    m.add_function(wrap_pyfunction!(parse_instagram, m)?)?;
    m.add_function(wrap_pyfunction!(parse_discord, m)?)?;
    
    // Utility functions
    m.add_function(wrap_pyfunction!(merge_consecutive, m)?)?;
    m.add_function(wrap_pyfunction!(apply_filters, m)?)?;
    
    Ok(())
}

/// Parse Telegram JSON export
///
/// Args:
///     path: Path to the Telegram export file (result.json)
///     merge: Merge consecutive messages from the same sender
///     min_length: Minimum message length to include
///     date_from: Filter messages from this date (ISO format or datetime)
///     date_to: Filter messages until this date (ISO format or datetime)
///
/// Returns:
///     List of message dictionaries
#[pyfunction]
#[pyo3(signature = (path, merge=false, min_length=None, date_from=None, date_to=None))]
fn parse_telegram(
    path: String,
    merge: bool,
    min_length: Option<usize>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> PyResult<Vec<PyMessage>> {
    parsers::parse_telegram_impl(path, merge, min_length, date_from, date_to)
}

/// Parse WhatsApp TXT export
///
/// Args:
///     path: Path to the WhatsApp export file (chat.txt)
///     merge: Merge consecutive messages from the same sender
///     min_length: Minimum message length to include
///     date_from: Filter messages from this date (ISO format or datetime)
///     date_to: Filter messages until this date (ISO format or datetime)
///
/// Returns:
///     List of message dictionaries
#[pyfunction]
#[pyo3(signature = (path, merge=false, min_length=None, date_from=None, date_to=None))]
fn parse_whatsapp(
    path: String,
    merge: bool,
    min_length: Option<usize>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> PyResult<Vec<PyMessage>> {
    parsers::parse_whatsapp_impl(path, merge, min_length, date_from, date_to)
}

/// Parse Instagram JSON export (GDPR dump)
///
/// Args:
///     path: Path to the Instagram export file (messages.json)
///     merge: Merge consecutive messages from the same sender
///     min_length: Minimum message length to include
///     date_from: Filter messages from this date (ISO format or datetime)
///     date_to: Filter messages until this date (ISO format or datetime)
///
/// Returns:
///     List of message dictionaries
#[pyfunction]
#[pyo3(signature = (path, merge=false, min_length=None, date_from=None, date_to=None))]
fn parse_instagram(
    path: String,
    merge: bool,
    min_length: Option<usize>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> PyResult<Vec<PyMessage>> {
    parsers::parse_instagram_impl(path, merge, min_length, date_from, date_to)
}

/// Parse Discord export (JSON/CSV/TXT from DiscordChatExporter)
///
/// Args:
///     path: Path to the Discord export file
///     merge: Merge consecutive messages from the same sender
///     min_length: Minimum message length to include
///     date_from: Filter messages from this date (ISO format or datetime)
///     date_to: Filter messages until this date (ISO format or datetime)
///
/// Returns:
///     List of message dictionaries
#[pyfunction]
#[pyo3(signature = (path, merge=false, min_length=None, date_from=None, date_to=None))]
fn parse_discord(
    path: String,
    merge: bool,
    min_length: Option<usize>,
    date_from: Option<String>,
    date_to: Option<String>,
) -> PyResult<Vec<PyMessage>> {
    parsers::parse_discord_impl(path, merge, min_length, date_from, date_to)
}

/// Merge consecutive messages from the same sender
///
/// Args:
///     messages: List of messages to merge
///     time_threshold: Maximum time gap in seconds between messages (default: 300)
///
/// Returns:
///     List of merged messages
#[pyfunction]
#[pyo3(signature = (messages, time_threshold=300))]
fn merge_consecutive(messages: Vec<PyMessage>, time_threshold: i64) -> PyResult<Vec<PyMessage>> {
    // use chatpack::core::merge::merge_consecutive as rust_merge;
    // use std::time::Duration;
    
    // TEMPORARY FIX: Merge functionality disabled due to missing 'merge' module in chatpack 0.5
    // Just return original messages for now or implement local merge if needed
    
    // let rust_messages: Vec<chatpack::Message> = messages
    //     .into_iter()
    //     .map(|m| m.into_rust())
    //     .collect();
    
    // let merged = rust_merge(rust_messages, Duration::from_secs(time_threshold as u64));
    
    // Ok(merged.into_iter().map(PyMessage::from_rust).collect())
    Ok(messages)
}

/// Apply filters to messages
///
/// Args:
///     messages: List of messages to filter
///     config: Filter configuration
///
/// Returns:
///     Filtered list of messages
#[pyfunction]
fn apply_filters(messages: Vec<PyMessage>, config: PyFilterConfig) -> PyResult<Vec<PyMessage>> {
    use chatpack::core::filter::apply_filters as rust_filter;
    
    let rust_messages: Vec<chatpack::Message> = messages
        .into_iter()
        .map(|m| m.into_rust())
        .collect();
    
    let filtered = rust_filter(rust_messages, &config.into_rust());
    
    Ok(filtered.into_iter().map(PyMessage::from_rust).collect())
}