"""
chatpack - High-performance chat export parser for Python

Parse and convert chat exports from Telegram, WhatsApp, Instagram, and Discord
into LLM-friendly formats. Built with Rust for maximum performance.

Example:
    >>> import chatpack
    >>> messages = chatpack.parse_telegram("result.json", merge=True)
    >>> import pandas as pd
    >>> df = pd.DataFrame([m.to_dict() for m in messages])
"""

from ._chatpack import (
    # Message types
    Message as PyMessage,
    FilterConfig as PyFilterConfig,
    OutputConfig as PyOutputConfig,
    
    # Parsers (classes)
    TelegramParser,
    WhatsAppParser,
    InstagramParser,
    DiscordParser,
    
    # Streaming parsers
    TelegramStreamParser,
    WhatsAppStreamParser,
    InstagramStreamParser,
    DiscordStreamParser,
    
    # Convenience functions
    parse_telegram,
    parse_whatsapp,
    parse_instagram,
    parse_discord,
    
    # Utilities
    merge_consecutive,
    apply_filters,
)

# Compatibility aliases
Message = PyMessage
FilterConfig = PyFilterConfig
OutputConfig = PyOutputConfig

__version__ = "0.1.0"

__all__ = [
    # Types
    "Message",
    "PyMessage",
    "FilterConfig",
    "PyFilterConfig",
    "OutputConfig",
    "PyOutputConfig",
    
    # Parsers
    "TelegramParser",
    "WhatsAppParser",
    "InstagramParser",
    "DiscordParser",
    
    # Streaming
    "TelegramStreamParser",
    "WhatsAppStreamParser",
    "InstagramStreamParser",
    "DiscordStreamParser",
    
    # Functions
    "parse_telegram",
    "parse_whatsapp",
    "parse_instagram",
    "parse_discord",
    "merge_consecutive",
    "apply_filters",
]