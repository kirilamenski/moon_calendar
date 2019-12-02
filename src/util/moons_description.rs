const MOON_NAMES: [&str; 12] = [
    "January - Wolf Moon",
    "February - Snow Moon",
    "March - Worm Moon",
    "April - Pink Moon",
    "May - Flower Moon",
    "June - Strawberry Moon",
    "July - Buck Moon",
    "August - Sturgeon Moon",
    "September - Full Corn Moon (Harvest)",
    "October - Hunter's Moon (Harvest)",
    "November - Beaver Moon",
    "December - Cold Moon",
];

pub fn get_moon_name(month: u32) -> &'static str {
    MOON_NAMES[(month - 1) as usize]
}