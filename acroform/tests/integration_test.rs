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
fn test_field_tooltip_extraction() {
    let doc = AcroFormDocument::from_pdf("../acroform_files/af8.pdf")
        .expect("Failed to load PDF");
    
    let fields = doc.fields().expect("Failed to get fields");
    
    // Find a field that should have a tooltip
    let field = fields.iter()
        .find(|f| f.name == "topmostSubform[0].Page1[0].P[0].MbrName[1]")
        .expect("Expected field not found");
    
    // Verify that the tooltip field is populated
    assert!(field.tooltip.is_some(), "Expected field to have a tooltip");
    assert_eq!(field.tooltip.as_ref().unwrap(), "MbrName");
    
    // Count fields with tooltips
    let tooltip_count = fields.iter().filter(|f| f.tooltip.is_some()).count();
    assert!(tooltip_count > 0, "Expected at least one field with a tooltip");
}

#[test]
fn test_field_default_value_extraction() {
    let doc = AcroFormDocument::from_pdf("../acroform_files/af8.pdf")
        .expect("Failed to load PDF");
    
    let fields = doc.fields().expect("Failed to get fields");
    
    // Verify that all fields have the default_value field available
    // (it may be None if no default value is set in the PDF)
    for field in &fields {
        // The default_value field should be accessible whether it's Some or None
        let _ = &field.default_value;
    }
    
    // Check that we can find a field by name and access its default_value
    if let Some(field) = fields.iter()
        .find(|f| f.name == "topmostSubform[0].Page1[0].P[0].MbrName[1]") {
        // The field should have a default_value field (may be Some or None)
        // This test primarily verifies the field is accessible
        let _default = &field.default_value;
    }
}

#[test]
fn test_need_appearances_flag_and_appearance_streams() {
    use pdf::file::FileOptions;
    
    let mut doc = AcroFormDocument::from_pdf("../acroform_files/af8_error.pdf")
        .expect("Failed to load PDF");
    
    // Update a field
    let mut values = HashMap::new();
    values.insert(
        "P[0].Page1[0].topmostSubform[0].MbrName[1]".to_string(),
        FieldValue::Text("NEW_VALUE".to_string()),
    );
    
    // Fill and save
    doc.fill_and_save(values, "/tmp/test_need_appearances.pdf")
        .expect("Failed to save PDF");
    
    // Load the filled PDF using the low-level API to check the flags
    let file = FileOptions::cached().open("/tmp/test_need_appearances.pdf")
        .expect("Failed to load filled PDF");
    
    // Verify Option 1: NeedAppearances flag is set to true
    if let Some(ref forms) = file.get_root().forms {
        assert!(
            forms.need_appearences,
            "NeedAppearances flag should be set to true"
        );
    } else {
        panic!("No AcroForm dictionary found");
    }
    
    // Verify Option 2: Appearance streams are removed
    let resolver = file.resolver();
    let mut found_updated_field = false;
    
    for page_rc in file.pages() {
        let page = page_rc.expect("Failed to get page");
        let annots = page.annotations.load(&resolver).expect("Failed to load annotations");
        
        for annot_ref in annots.data().iter() {
            let annot = annot_ref.data();
            
            // Check if this is the MbrName[1] field
            if let Some(pdf::primitive::Primitive::String(ref field_name)) = annot.other.get("T") {
                let field_name_str = field_name.to_string_lossy().to_string();
                
                if field_name_str == "MbrName[1]" {
                    found_updated_field = true;
                    
                    // Verify appearance streams were removed
                    assert!(
                        annot.appearance_streams.is_none(),
                        "Appearance streams should be removed from updated annotations"
                    );
                    break;
                }
            }
        }
        
        if found_updated_field {
            break;
        }
    }
    
    assert!(found_updated_field, "Should have found the updated field");
}
