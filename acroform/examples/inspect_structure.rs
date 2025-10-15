use pdf::file::FileOptions;
use pdf::object::{Resolve, RcRef, FieldDictionary};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = FileOptions::cached().open("../acroform_files/af8.pdf")?;
    let resolver = file.resolver();
    
    println!("=== Inspecting PDF Structure ===\n");
    
    // Get the form
    if let Some(ref forms) = file.get_root().forms {
        println!("Found AcroForm dictionary");
        println!("Number of top-level fields: {}", forms.fields.len());
        
        // Iterate through all fields
        for (i, field_ref) in forms.fields.iter().enumerate() {
            let field: RcRef<FieldDictionary> = resolver.get(field_ref.get_ref())?;
            println!("\nField {}: {:?}", i, field_ref.get_ref());
            if let Some(ref name) = field.name {
                println!("  Name: {}", name.to_string_lossy());
            }
            if let Some(ref typ) = field.typ {
                println!("  Type: {:?}", typ);
            }
            println!("  Value: {:?}", field.value);
            println!("  Kids count: {}", field.kids.len());
            
            // Check if this field has a Rect (which would make it a widget)
            if let Some(ref rect) = field.rect {
                println!("  Has Rect (is a widget): {:?}", rect);
            }
            
            // Check for subtype
            if let Some(ref subtype) = field.subtype {
                println!("  Subtype: {}", subtype);
            }
            
            // Traverse kids
            for (j, kid_ref) in field.kids.iter().enumerate() {
                let kid: RcRef<FieldDictionary> = resolver.get(*kid_ref)?;
                println!("    Kid {}: {:?}", j, kid_ref);
                if let Some(ref name) = kid.name {
                    println!("      Name: {}", name.to_string_lossy());
                }
                if let Some(ref typ) = kid.typ {
                    println!("      Type: {:?}", typ);
                }
                println!("      Value: {:?}", kid.value);
                if let Some(ref rect) = kid.rect {
                    println!("      Has Rect (is a widget): {:?}", rect);
                }
                if let Some(ref subtype) = kid.subtype {
                    println!("      Subtype: {}", subtype);
                }
            }
        }
    }
    
    // Now check page annotations
    println!("\n\n=== Page Annotations ===\n");
    for (page_num, page_rc) in file.pages().enumerate() {
        let page = page_rc?;
        let annots = page.annotations.load(&resolver)?;
        println!("Page {}: {} annotations", page_num, annots.len());
        
        for (i, annot_ref) in annots.iter().enumerate() {
            let annot = annot_ref.data();
            
            // Try to get the reference if it's indirect
            if let Some(ref_val) = annot_ref.as_ref() {
                println!("  Annotation {}: Ref = {:?}, Subtype = {}", i, ref_val, annot.subtype);
            } else {
                println!("  Annotation {}: (Direct), Subtype = {}", i, annot.subtype);
            }
            
            if let Some(ref name) = annot.annotation_name {
                println!("    Annotation Name: {}", name.to_string_lossy());
            }
            
            // Check if this annotation has field dictionary properties
            if let Some(ref dict) = annot.other.get("T") {
                println!("    Has T (field name): {:?}", dict);
            }
            if let Some(ref dict) = annot.other.get("FT") {
                println!("    Has FT (field type): {:?}", dict);
            }
            if let Some(ref dict) = annot.other.get("V") {
                println!("    Has V (value): {:?}", dict);
            }
            if let Some(ref dict) = annot.other.get("Parent") {
                println!("    Has Parent: {:?}", dict);
            }
        }
    }
    
    Ok(())
}
