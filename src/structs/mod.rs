use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ImageDimensions {
    pub tiny: Option<Dimensions>,
    pub large: Option<Dimensions>,
    pub small: Option<Dimensions>,
    pub medium: Option<Dimensions>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Dimensions {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    pub tiny: Option<String>,
    pub large: Option<String>,
    pub small: Option<String>,
    pub medium: Option<String>,
    pub original: Option<String>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Meta {
    pub dimensions: Option<ImageDimensions>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Titles {
    pub en: Option<String>,
    pub en_jp: Option<String>,
    pub ja_jp: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RatingFrequencies {
    #[serde(rename = "2")]
    pub two: Option<String>,
    #[serde(rename = "3")]
    pub three: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct AnimeAttributes {
    pub createdAt: Option<String>,
    pub updatedAt: Option<String>,
    pub slug: Option<String>,
    pub synopsis: Option<String>,
    pub description: Option<String>,
    pub coverImageTopOffset: Option<u32>,
    pub titles: Option<Titles>,
    pub canonicalTitle: Option<String>,
    pub abbreviatedTitles: Option<Vec<String>>,
    pub averageRating: Option<String>,
    pub ratingFrequencies: Option<RatingFrequencies>,
    pub userCount: Option<u32>,
    pub favoritesCount: Option<u32>,
    pub startDate: Option<String>,
    pub endDate: Option<String>,
    pub popularityRank: Option<u32>,
    pub ratingRank: Option<u32>,
    pub ageRating: Option<String>,
    pub ageRatingGuide: Option<String>,
    pub subtype: Option<String>,
    pub status: Option<String>,
    pub posterImage: Option<Image>,
    pub coverImage: Option<Image>,
    pub episodeCount: Option<u32>,
    pub episodeLength: Option<u32>,
    pub totalLength: Option<u32>,
    pub youtubeVideoId: Option<String>,
    pub showType: Option<String>,
    pub nsfw: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_: Option<String>,
    pub related: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RelationshipsField {
    pub links: Option<Links>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct Relationships {
    pub genres: Option<RelationshipsField>,
    pub categories: Option<RelationshipsField>,
    pub castings: Option<RelationshipsField>,
    pub installments: Option<RelationshipsField>,
    pub mappings: Option<RelationshipsField>,
    pub reviews: Option<RelationshipsField>,
    pub mediaRelationships: Option<RelationshipsField>,
    pub characters: Option<RelationshipsField>,
    pub staff: Option<RelationshipsField>,
    pub productions: Option<RelationshipsField>,
    pub quotes: Option<RelationshipsField>,
    pub episodes: Option<RelationshipsField>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnimeData {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub attributes: Option<AnimeAttributes>,
    pub relationships: Option<Relationships>,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub data: Option<Vec<AnimeData>>,
}
