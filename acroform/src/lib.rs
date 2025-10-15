/*!
# acroform - PDF Form Manipulation

A library for reading and filling PDF forms programmatically.

## Quick Start

```rust,no_run
use acroform::{AcroFormDocument, FieldValue};
use std::collections::HashMap;

// Load a PDF with a form
let mut doc = AcroFormDocument::from_pdf("form.pdf").unwrap();

// List all form fields
for field in doc.fields().unwrap() {
    println!("Field: {} = {:?}", field.name, field.current_value);
}

// Fill in form fields
let mut values = HashMap::new();
values.insert("firstName".to_string(), FieldValue::Text("John".to_string()));
values.insert("lastName".to_string(), FieldValue::Text("Doe".to_string()));

// Save the filled form to a new file
doc.fill_and_save(values, "filled_form.pdf").unwrap();
```

## Working with Form Fields

This library supports the following field types:
- **Text fields** - Use `FieldValue::Text(String)`
- **Checkboxes** - Use `FieldValue::Boolean(bool)`
- **Radio buttons and dropdowns** - Use `FieldValue::Choice(String)`
- **Number fields** - Use `FieldValue::Integer(i32)`

Field names are fully qualified (e.g., `"parent.child.field"`) and automatically
resolved for you, even in forms with nested field hierarchies.
*/

mod field;
mod api;

pub use api::{AcroFormDocument, FormField, FieldValue};
pub use field::{FieldDictionaryExt, InteractiveFormDictionaryExt};

// Re-export commonly used types from pdf crate
pub use pdf::error::PdfError;
pub use pdf::object::FieldType;
