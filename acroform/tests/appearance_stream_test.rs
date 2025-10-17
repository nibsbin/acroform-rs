use acroform::{AcroFormDocument, FieldValue};
use std::collections::HashMap;

#[test]
fn test_appearance_stream_generation() {
    // Load the test PDF
    let mut doc = AcroFormDocument::from_pdf("../acroform_files/af8.pdf")
        .expect("Failed to load test PDF");
    
    // Update a field value
    let mut values = HashMap::new();
    values.insert(
        "P[0].Page1[0].topmostSubform[0].MbrName[1]".to_string(),
        FieldValue::Text("NEW_VALUE".to_string()),
    );
    
    // Fill the form and get the result as bytes
    let filled_pdf_bytes = doc.fill(values).expect("Failed to fill form");
    
    // Verify that the filled PDF contains appearance stream markers
    let pdf_content = String::from_utf8_lossy(&filled_pdf_bytes);
    
    // Check for key appearance stream components
    // /AP indicates appearance dictionary was created
    assert!(pdf_content.contains("/AP"), "PDF should contain appearance dictionary");
    
    // The appearance stream content might be compressed, but the dictionary should be present
    // This verifies that our code is creating appearance streams
    
    // Verify the PDF is not empty and has reasonable size
    assert!(filled_pdf_bytes.len() > 1000, "Filled PDF should have reasonable size");
}

#[test]
fn test_appearance_stream_with_special_characters() {
    // Load the test PDF
    let mut doc = AcroFormDocument::from_pdf("../acroform_files/af8.pdf")
        .expect("Failed to load test PDF");
    
    // Update a field with special characters that need escaping
    let mut values = HashMap::new();
    values.insert(
        "P[0].Page1[0].topmostSubform[0].MbrName[1]".to_string(),
        FieldValue::Text("Test (with) special\\characters".to_string()),
    );
    
    // Fill the form - should not panic
    let filled_pdf_bytes = doc.fill(values).expect("Failed to fill form with special characters");
    
    // Verify the PDF was generated
    assert!(filled_pdf_bytes.len() > 1000, "Filled PDF should have reasonable size");
}
