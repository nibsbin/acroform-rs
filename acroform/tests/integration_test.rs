use acroform::{AcroFormDocument, FieldValue};
use std::collections::HashMap;

#[test]
fn test_load_and_list_fields() {
    let doc = AcroFormDocument::from_pdf("../acroform_files/af8.pdf")
        .expect("Failed to load PDF");
    
    let fields = doc.fields().expect("Failed to get fields");
    
    // The test PDF should have at least one field
    assert!(!fields.is_empty(), "Expected at least one field in the PDF");
    
    // Check that we can find the expected field
    let field_names: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();
    assert!(
        field_names.contains(&"topmostSubform[0].Page1[0].P[0].MbrName[1]".to_string()),
        "Expected to find MbrName field"
    );
}

#[test]
fn test_fill_and_save() {
    let mut doc = AcroFormDocument::from_pdf("../acroform_files/af8.pdf")
        .expect("Failed to load PDF");
    
    // Get initial field value
    let fields = doc.fields().expect("Failed to get fields");
    let initial_field = fields.iter()
        .find(|f| f.name == "topmostSubform[0].Page1[0].P[0].MbrName[1]")
        .expect("Field not found");
    
    assert_eq!(
        initial_field.current_value,
        Some(FieldValue::Text("OLD_VALUE".to_string()))
    );
    
    // Update the field
    let mut values = HashMap::new();
    values.insert(
        "topmostSubform[0].Page1[0].P[0].MbrName[1]".to_string(),
        FieldValue::Text("TEST_VALUE".to_string()),
    );
    
    doc.fill_and_save(values, "/tmp/test_filled.pdf")
        .expect("Failed to save PDF");
    
    // Reload and verify
    let doc2 = AcroFormDocument::from_pdf("/tmp/test_filled.pdf")
        .expect("Failed to reload PDF");
    
    let fields2 = doc2.fields().expect("Failed to get fields");
    let updated_field = fields2.iter()
        .find(|f| f.name == "topmostSubform[0].Page1[0].P[0].MbrName[1]")
        .expect("Field not found after update");
    
    assert_eq!(
        updated_field.current_value,
        Some(FieldValue::Text("TEST_VALUE".to_string()))
    );
}

#[test]
fn test_nonexistent_field() {
    let mut doc = AcroFormDocument::from_pdf("../acroform_files/af8.pdf")
        .expect("Failed to load PDF");
    
    let mut values = HashMap::new();
    values.insert(
        "nonexistent.field".to_string(),
        FieldValue::Text("value".to_string()),
    );
    
    // Should not error when trying to update a nonexistent field
    // (it just won't update anything)
    doc.fill_and_save(values, "/tmp/test_nonexistent.pdf")
        .expect("Should not error on nonexistent field");
}

#[test]
fn test_from_bytes() {
    // Load from file as bytes
    let data = std::fs::read("../acroform_files/af8.pdf")
        .expect("Failed to read PDF file");
    
    // Load from bytes
    let doc = AcroFormDocument::from_bytes(data)
        .expect("Failed to load PDF from bytes");
    
    // Verify we can read fields
    let fields = doc.fields().expect("Failed to get fields");
    assert!(!fields.is_empty(), "Expected at least one field in the PDF");
    
    // Check that we can find the expected field
    let field_names: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();
    assert!(
        field_names.contains(&"topmostSubform[0].Page1[0].P[0].MbrName[1]".to_string()),
        "Expected to find MbrName field"
    );
}

#[test]
fn test_fill_in_memory() {
    let mut doc = AcroFormDocument::from_pdf("../acroform_files/af8.pdf")
        .expect("Failed to load PDF");
    
    // Update the field
    let mut values = HashMap::new();
    values.insert(
        "topmostSubform[0].Page1[0].P[0].MbrName[1]".to_string(),
        FieldValue::Text("IN_MEMORY_VALUE".to_string()),
    );
    
    // Get filled PDF as bytes
    let filled_bytes = doc.fill(values)
        .expect("Failed to fill PDF");
    
    // Verify we got bytes back
    assert!(!filled_bytes.is_empty(), "Expected non-empty PDF bytes");
    
    // Load the bytes and verify the field was updated
    let doc2 = AcroFormDocument::from_bytes(filled_bytes)
        .expect("Failed to load filled PDF from bytes");
    
    let fields2 = doc2.fields().expect("Failed to get fields");
    let updated_field = fields2.iter()
        .find(|f| f.name == "topmostSubform[0].Page1[0].P[0].MbrName[1]")
        .expect("Field not found after update");
    
    assert_eq!(
        updated_field.current_value,
        Some(FieldValue::Text("IN_MEMORY_VALUE".to_string()))
    );
}

#[test]
fn test_round_trip_in_memory() {
    // Read PDF as bytes
    let original_data = std::fs::read("../acroform_files/af8.pdf")
        .expect("Failed to read PDF file");
    
    // Load from bytes
    let mut doc = AcroFormDocument::from_bytes(original_data.clone())
        .expect("Failed to load PDF from bytes");
    
    // Fill some fields
    let mut values = HashMap::new();
    values.insert(
        "topmostSubform[0].Page1[0].P[0].MbrName[1]".to_string(),
        FieldValue::Text("ROUND_TRIP_TEST".to_string()),
    );
    
    // Get filled PDF as bytes (in-memory operation)
    let filled_bytes = doc.fill(values)
        .expect("Failed to fill PDF");
    
    // Load the filled bytes
    let final_doc = AcroFormDocument::from_bytes(filled_bytes)
        .expect("Failed to load final PDF from bytes");
    
    // Verify the update persisted
    let fields = final_doc.fields().expect("Failed to get fields");
    let updated_field = fields.iter()
        .find(|f| f.name == "topmostSubform[0].Page1[0].P[0].MbrName[1]")
        .expect("Field not found");
    
    assert_eq!(
        updated_field.current_value,
        Some(FieldValue::Text("ROUND_TRIP_TEST".to_string())),
        "Field value should be updated after round trip"
    );
}

#[test]
fn test_unicode_characters_in_form_fields() {
    // Test that Unicode characters (smart quotes, accented characters, etc.) 
    // are correctly encoded when filling form fields
    let mut doc = AcroFormDocument::from_pdf("../acroform_files/af8.pdf")
        .expect("Failed to load PDF");
    
    // Test various Unicode characters that should be handled by PDFDocEncoding
    let test_values = vec![
        ("Smart quotes", "\u{201C}Hello\u{201D}"),  // Left and right double quotes
        ("Single quotes", "\u{2018}test\u{2019}"),   // Left and right single quotes
        ("Em dash", "test\u{2014}dash"),             // Em dash
        ("En dash", "test\u{2013}dash"),             // En dash
        ("Accented", "caf\u{00E9}"),                 // Café with accented e
        ("Trademark", "Test\u{2122}"),               // Trademark symbol
        ("Copyright", "\u{00A9}2024"),               // Copyright symbol
    ];
    
    for (description, value) in test_values {
        let mut values = HashMap::new();
        values.insert(
            "topmostSubform[0].Page1[0].P[0].MbrName[1]".to_string(),
            FieldValue::Text(value.to_string()),
        );
        
        // Fill the form
        let filled_bytes = doc.fill(values)
            .expect(&format!("Failed to fill PDF with {}", description));
        
        // Verify we got bytes back
        assert!(!filled_bytes.is_empty(), "Expected non-empty PDF bytes for {}", description);
        
        // Load the filled PDF and verify the value
        let doc2 = AcroFormDocument::from_bytes(filled_bytes)
            .expect(&format!("Failed to load filled PDF for {}", description));
        
        let fields2 = doc2.fields().expect("Failed to get fields");
        let updated_field = fields2.iter()
            .find(|f| f.name == "topmostSubform[0].Page1[0].P[0].MbrName[1]")
            .expect(&format!("Field not found after update for {}", description));
        
        // The value should match what we put in
        assert_eq!(
            updated_field.current_value,
            Some(FieldValue::Text(value.to_string())),
            "Field value mismatch for {}. Expected '{}', got '{:?}'",
            description,
            value,
            updated_field.current_value
        );
    }
}
