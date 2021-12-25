extern crate osmpbf;

mod importer;

fn main() {
    importer::import().unwrap();
}
