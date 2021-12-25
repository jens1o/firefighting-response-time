use osmpbf::{Element, ElementReader};

pub mod error;

pub fn import() -> Result<(), error::ImporterError> {
    let reader = ElementReader::from_path("karlsruhe-regbez-latest.osm.pbf")?;

    let ways = reader.par_map_reduce(
        |element| match element {
            Element::Way(_) => 1,
            _ => 0,
        },
        || 0_u64,     // Zero is the identity value for addition
        |a, b| a + b, // Sum the partial results
    )?;

    println!("Number of ways: {}", ways);

    Ok(())
}
