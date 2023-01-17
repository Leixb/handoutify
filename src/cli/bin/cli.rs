use lopdf::Document;

use std::error::Error;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "handoutify", about = "Remove pauses from beamer slides.")]
struct Opt {
    /// The PDF file to read
    #[structopt(name = "PDF_FILE")]
    file: String,

    /// The output file (default: <PDF_FILE>_handout.pdf)
    #[structopt(short, long)]
    output: Option<String>,

    /// Optimize PDF output
    #[structopt(short, long)]
    prune: bool,

    /// Renumber PDF objects
    #[structopt(short, long)]
    renumber: bool,

    /// Overwrite output file if it exists
    #[structopt(long)]
    overwrite: bool,
}

fn convert(
    filename: String,
    filename_out: String,
    overwrite: bool,
    prune_objects: bool,
    renumber_objects: bool,
) -> Result<(), Box<dyn Error>> {
    let mut doc = Document::load(filename).unwrap();

    if let Err(e) = handoutify::handoutify(&mut doc) {
        println!("Error: {}", e);
        return Ok(());
    }

    if prune_objects {
        doc.prune_objects();
    }
    if renumber_objects {
        doc.renumber_objects();
    }

    if overwrite || !std::path::Path::new(&filename_out).exists() {
        doc.save(filename_out).unwrap();
        Ok(())
    } else {
        println!("File {} already exists, not overwriting", filename_out);
        Ok(())
    }
}

fn main() {
    let opt = Opt::from_args();
    let filename_out = opt.output.unwrap_or(
        opt.file
            .trim_end_matches(".pdf")
            .to_string()
            .clone()
            .to_owned()
            + "_handout.pdf",
    );
    convert(
        opt.file,
        filename_out,
        opt.overwrite,
        opt.prune,
        opt.renumber,
    )
    .unwrap();
}
