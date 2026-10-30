use innertube::{self, Innertube};

fn main() -> eyre::Result<()> {
    let innertube = Innertube::new().unwrap();
    dbg!(innertube.channel_metadata("UCX6OQ3DkcsbYNE6H8uQQuVA")?);
    Ok(())
}
