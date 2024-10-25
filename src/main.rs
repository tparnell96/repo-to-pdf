use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    // Create default values with extended lifetimes
    let default_dir = ".".to_string();
    let default_extension = ".py".to_string();
    let default_output_pdf = "output.pdf".to_string();

    let dir = args.get(1).unwrap_or(&default_dir);
    let extension_arg = args.get(2).unwrap_or(&default_extension);
    let output_pdf = args.get(3).unwrap_or(&default_output_pdf);

    // Collect exclude directories from the remaining arguments
    let exclude_dirs: Vec<&str> = args.iter().skip(4).map(|s| s.as_str()).collect();

    println!("Directory: {}", dir);
    println!("Extension: {}", extension_arg);
    println!("Output PDF: {}", output_pdf);
    if !exclude_dirs.is_empty() {
        println!("Excluding directories: {:?}", exclude_dirs);
    }

    // Remove leading '.' from extension if present
    let extension = if extension_arg.starts_with('.') {
        &extension_arg[1..]
    } else {
        extension_arg.as_str()
    };

    // Collect file paths
    let mut file_paths = Vec::new();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_excluded(e.path(), &exclude_dirs))
    {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Error reading entry: {}", err);
                continue;
            }
        };
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.to_string_lossy() == extension {
                    file_paths.push(path.to_owned());
                }
            }
        }
    }

    // Read file contents
    let mut contents = Vec::new();
    for path in file_paths {
        if let Ok(content) = fs::read_to_string(&path) {
            contents.push((path, content));
        } else {
            eprintln!("Failed to read file: {:?}", path);
        }
    }

    // Generate PDF
    generate_pdf(&contents, output_pdf);
}

fn is_excluded(path: &Path, exclude_dirs: &Vec<&str>) -> bool {
    for component in path.components() {
        if let std::path::Component::Normal(os_str) = component {
            if let Some(dir_name) = os_str.to_str() {
                if exclude_dirs.contains(&dir_name) {
                    return true;
                }
            }
        }
    }
    false
}

fn generate_pdf(contents: &Vec<(PathBuf, String)>, output_pdf: &str) {
    use printpdf::*;
    use std::io::BufWriter;

    // Define page size and margins
    let (page_width, page_height) = (Mm(210.0), Mm(297.0)); // A4 size

    // Create a new PDF document
    let (doc, page1, layer1) = PdfDocument::new("Repository Files", page_width, page_height, "Layer 1");

    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let monospace_font = doc.add_builtin_font(BuiltinFont::Courier).unwrap();

    let mut current_layer = doc.get_page(page1).get_layer(layer1);

    let font_size_heading = 18.0;
    let font_size_content = 12.0;

    let mut current_y = page_height - Mm(20.0); // Start 20mm from top

    for (path, content) in contents {
        // Reset current_y if starting a new page
        if current_y < Mm(40.0) {
            // Start a new page
            let (page, layer) = doc.add_page(page_width, page_height, "Layer 1");
            current_layer = doc.get_page(page).get_layer(layer);
            current_y = page_height - Mm(20.0);
        }

        // Draw heading
        let text_heading = format!("File: {}", path.display());
        current_layer.use_text(
            text_heading,
            font_size_heading,
            Mm(20.0),
            current_y,
            &font,
        );

        current_y -= Mm(10.0);

        // Draw content
        let code_lines = content.lines();
        for line in code_lines {
            if current_y < Mm(20.0) {
                // Start a new page if we've reached the bottom margin
                let (page, layer) = doc.add_page(page_width, page_height, "Layer 1");
                current_layer = doc.get_page(page).get_layer(layer);
                current_y = page_height - Mm(20.0);
            }

            current_layer.use_text(
                line,
                font_size_content,
                Mm(20.0),
                current_y,
                &monospace_font,
            );

            current_y -= Mm(5.0); // Move down by 5mm per line
        }

        // Add some spacing before the next file
        current_y -= Mm(15.0);
    }

    // Save the PDF document
    let file = fs::File::create(output_pdf).expect("Couldn't create output file");
    let mut buf_writer = BufWriter::new(file);
    doc.save(&mut buf_writer).expect("Couldn't save PDF");
}
