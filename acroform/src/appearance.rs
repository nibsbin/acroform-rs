use pdf::content::FormXObject;
use pdf::object::{FieldDictionary, Resources, Resolve, FormDict, AppearanceStreams, AppearanceStreamEntry, Stream, Rectangle};
use pdf::error::PdfError;
use std::sync::Arc;

use crate::FieldValue;

/// Generate appearance stream for a text field
/// 
/// This function creates a PDF appearance stream that visually represents
/// the field's value. The appearance stream contains PDF drawing operators
/// that render the text.
pub fn generate_text_appearance(
    field: &FieldDictionary,
    value: &str,
    _resolve: &impl Resolve,
) -> Result<FormXObject, PdfError> {
    // Get the field's rectangle (required for appearance stream)
    let rect = field.rect.ok_or_else(|| PdfError::MissingEntry {
        typ: "Field",
        field: "Rect".into(),
    })?;
    
    // Calculate dimensions
    let width = rect.right - rect.left;
    let height = rect.top - rect.bottom;
    
    // Default font size (10pt is a common default)
    let font_size = 10.0;
    
    // Vertical position - center the text vertically with some padding
    let y_position = (height - font_size) / 2.0;
    
    // Horizontal padding
    let x_padding = 2.0;
    
    // Generate appearance stream content using PDF operators
    // This is a simplified version that works for most cases
    let content = generate_text_appearance_ops(
        value,
        width,
        height,
        x_padding,
        y_position,
        font_size,
    )?;
    
    // Create the FormDict for the appearance stream
    let form_dict = FormDict {
        form_type: 1,
        name: None,
        last_modified: None,
        bbox: Rectangle {
            left: 0.0,
            bottom: 0.0,
            right: width,
            top: height,
        },
        matrix: None,
        resources: Some(pdf::object::MaybeRef::Direct(Arc::new(create_default_resources()))),
        group: None,
        reference: None,
        metadata: None,
        piece_info: None,
        struct_parent: None,
        struct_parents: None,
        opi: None,
        other: pdf::primitive::Dictionary::new(),
    };
    
    // Create the stream with the content
    let stream = Stream::new(form_dict, content);
    
    Ok(FormXObject { stream })
}

/// Generate PDF drawing operators for text appearance
fn generate_text_appearance_ops(
    text: &str,
    width: f32,
    height: f32,
    x_pos: f32,
    y_pos: f32,
    font_size: f32,
) -> Result<Vec<u8>, PdfError> {
    use std::fmt::Write;
    
    let mut content = String::new();
    
    // Begin marked content for text field
    writeln!(content, "/Tx BMC").map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    // Save graphics state
    writeln!(content, "q").map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    // Set clipping rectangle to field bounds
    writeln!(content, "0 0 {} {} re", width, height).map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    writeln!(content, "W").map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    writeln!(content, "n").map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    // Begin text object
    writeln!(content, "BT").map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    // Set font and size (using Helvetica as default, which is a standard PDF font)
    writeln!(content, "/Helv {} Tf", font_size).map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    // Set text color to black
    writeln!(content, "0 g").map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    // Position text
    writeln!(content, "{} {} Td", x_pos, y_pos).map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    // Show text - escape special characters
    let escaped_text = escape_pdf_string(text);
    writeln!(content, "({}) Tj", escaped_text).map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    // End text object
    writeln!(content, "ET").map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    // Restore graphics state
    writeln!(content, "Q").map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    // End marked content
    writeln!(content, "EMC").map_err(|e| PdfError::Encoding { source: Box::new(e) })?;
    
    Ok(content.into_bytes())
}

/// Escape special characters in PDF strings
fn escape_pdf_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '(' => result.push_str("\\("),
            ')' => result.push_str("\\)"),
            '\\' => result.push_str("\\\\"),
            '\r' => result.push_str("\\r"),
            '\n' => result.push_str("\\n"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
}

/// Create default resources for appearance streams
/// 
/// This includes standard PDF fonts (Helvetica, Times, etc.) that should
/// be available in all PDF viewers.
fn create_default_resources() -> Resources {
    // For now, we return an empty Resources dictionary
    // The PDF viewer should use the default appearance string (DA) from the
    // field or AcroForm dictionary to determine the font
    // This is simpler and more compatible with existing PDFs
    Resources::default()
}

/// Generate appearance streams for a field based on its value
pub fn generate_appearance_stream(
    field: &FieldDictionary,
    value: &FieldValue,
    resolve: &impl Resolve,
) -> Result<Option<AppearanceStreams>, PdfError> {
    // Only generate appearance streams for text fields for now
    // Other field types (buttons, checkboxes, etc.) would need different handling
    match value {
        FieldValue::Text(text) => {
            let form_xobject = generate_text_appearance(field, text, resolve)?;
            
            // Create appearance streams with only the normal appearance
            Ok(Some(AppearanceStreams {
                normal: pdf::object::MaybeRef::Direct(Arc::new(AppearanceStreamEntry::Single(form_xobject))),
                rollover: None,
                down: None,
            }))
        }
        // For other field types, we don't generate appearance streams yet
        _ => Ok(None),
    }
}
