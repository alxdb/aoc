use std::error::Error;

pub mod solutions;

#[derive(Copy, Clone, Debug)]
pub struct AocId {
    pub year: u16,
    pub day: u8,
    pub part: u8,
}

pub fn fetch_input(aoc_id: AocId) -> Result<String, Box<dyn Error>> {
    let input_file = dirs::cache_dir()
        .ok_or("Users cache directory cannot be determined")?
        .join("aoc")
        .join(format!("input_{:02}_{:02}", aoc_id.year, aoc_id.day));

    let input = if input_file.exists() {
        std::fs::read_to_string(input_file)?
    } else {
        let url = format!(
            "https://adventofcode.com/{}/day/{}/input",
            aoc_id.year, aoc_id.day,
        );
        let cookie = format!("session={}", std::env::var("AOC_TOKEN")?);

        let input = ureq::get(&url)
            .set("COOKIE", &cookie)
            .call()?
            .into_string()?;

        std::fs::write(input_file, &input)?;
        input
    };

    Ok(input)
}
