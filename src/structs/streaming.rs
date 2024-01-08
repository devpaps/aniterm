use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiStreamingResponse {
    pub data: Option<Vec<StreamingLinkData>>,
    pub meta: Option<MetaData>,
    pub links: Option<LinksData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StreamingLinkData {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub links: Option<Link>,
    pub attributes: Option<StreamingLinkAttributes>,
    pub relationships: Option<StreamingLinkRelationships>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetaData {
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LinksData {
    pub first: Option<String>,
    pub last: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Link {
    #[serde(rename = "self")]
    pub self_link: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StreamingLinkAttributes {
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    pub url: Option<String>,
    pub subs: Option<Vec<String>>,
    pub dubs: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StreamingLinkRelationships {
    pub streamer: Option<RelationshipField>,
    pub media: Option<RelationshipField>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RelationshipField {
    pub links: Option<Link>,
}
