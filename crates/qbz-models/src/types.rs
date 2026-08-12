//! Core API types for QBZ
//!
//! This module contains all shared data types used across the application:
//! - Media types: Track, Album, Artist, Playlist
//! - Quality/streaming types
//! - Search and favorites types
//! - Image and metadata types

use serde::{Deserialize, Serialize};

// ============ Quality Types ============

/// Audio quality format IDs (matches Qobuz API format IDs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[repr(u32)]
pub enum Quality {
    Mp3 = 5,
    #[default]
    Lossless = 6,    // 16-bit/44.1kHz (CD Quality)
    HiRes = 7,       // 24-bit/≤96kHz
    UltraHiRes = 27, // 24-bit/>96kHz
}

impl Quality {
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            5 => Some(Quality::Mp3),
            6 => Some(Quality::Lossless),
            7 => Some(Quality::HiRes),
            27 => Some(Quality::UltraHiRes),
            _ => None,
        }
    }

    pub fn id(&self) -> u32 {
        *self as u32
    }

    pub fn label(&self) -> &'static str {
        match self {
            Quality::Mp3 => "MP3 320kbps",
            Quality::Lossless => "FLAC 16-bit/44.1kHz",
            Quality::HiRes => "FLAC 24-bit/≤96kHz",
            Quality::UltraHiRes => "FLAC 24-bit/>96kHz",
        }
    }

    /// Quality levels in descending order for fallback
    pub fn fallback_order() -> &'static [Quality] {
        &[
            Quality::UltraHiRes,
            Quality::HiRes,
            Quality::Lossless,
            Quality::Mp3,
        ]
    }

    /// Returns the next lower quality level, or None if already at the lowest (Mp3).
    /// Used for CDN fallback when a quality level consistently fails.
    pub fn lower(&self) -> Option<Quality> {
        match self {
            Quality::UltraHiRes => Some(Quality::HiRes),
            Quality::HiRes => Some(Quality::Lossless),
            Quality::Lossless => Some(Quality::Mp3),
            Quality::Mp3 => None,
        }
    }
}



// ============ User Session ============

/// User credentials and session info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub user_auth_token: String,
    pub user_id: u64,
    pub email: String,
    pub display_name: String,
    pub subscription_label: String,
    #[serde(default)]
    pub subscription_valid_until: Option<String>,
}

// ============ Stream Types ============

/// Stream URL response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamUrl {
    pub url: String,
    pub format_id: u32,
    pub mime_type: String,
    pub sampling_rate: f64,
    pub bit_depth: Option<u32>,
    pub track_id: u64,
    pub restrictions: Vec<StreamRestriction>,
}

impl StreamUrl {
    /// Check if the stream has restrictions that prevent playback
    pub fn has_restrictions(&self) -> bool {
        self.restrictions.iter().any(|r| {
            r.code == "FormatRestrictedByFormatAvailability"
                || r.code == "SampleRestrictedByRightHolders"
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRestriction {
    pub code: String,
}

// ============ CMAF Stream Types ============

// ============ Image Types ============

/// Image set with multiple resolutions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageSet {
    pub small: Option<String>,
    pub thumbnail: Option<String>,
    pub large: Option<String>,
    pub extralarge: Option<String>,
    pub mega: Option<String>,
    pub back: Option<String>,
}

impl ImageSet {
    pub fn best(&self) -> Option<&String> {
        self.mega
            .as_ref()
            .or(self.extralarge.as_ref())
            .or(self.large.as_ref())
            .or(self.thumbnail.as_ref())
            .or(self.small.as_ref())
    }
}

// ============ Core Media Types ============

/// Track model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub title: String,
    /// Subtitle/edition info from Qobuz (e.g. "Player's Ball Mix",
    /// "Nine Inch Noize Version", "Remastered 2024"). Frontend renders
    /// it parenthesized after the title so remix and reissue albums are
    /// distinguishable from originals (issue #360).
    pub version: Option<String>,
    pub isrc: Option<String>,
    #[serde(default)]
    pub duration: u32,
    #[serde(default)]
    pub track_number: u32,
    pub media_number: Option<u32>,
    pub performer: Option<Artist>,
    pub album: Option<AlbumSummary>,
    #[serde(default)]
    pub hires: bool,
    #[serde(default)]
    pub hires_streamable: bool,
    pub maximum_sampling_rate: Option<f64>,
    pub maximum_bit_depth: Option<u32>,
    #[serde(default)]
    pub streamable: bool,
    #[serde(default)]
    pub parental_warning: bool,
    /// Playlist-specific: ID within the playlist (for removal)
    pub playlist_track_id: Option<u64>,
    /// Performers/credits string (format: "Name, Role - Name, Role")
    pub performers: Option<String>,
    /// Composer information
    pub composer: Option<Artist>,
    /// Copyright information
    pub copyright: Option<String>,
}

/// Album summary (embedded in track responses)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumSummary {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub image: ImageSet,
    /// Label (if returned in track response)
    pub label: Option<Label>,
}

/// Album model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub artist: Artist,
    #[serde(default)]
    pub image: ImageSet,
    pub release_date_original: Option<String>,
    pub label: Option<Label>,
    pub genre: Option<Genre>,
    pub tracks_count: Option<u32>,
    pub duration: Option<u32>,
    #[serde(default)]
    pub hires: bool,
    #[serde(default)]
    pub hires_streamable: bool,
    pub maximum_sampling_rate: Option<f64>,
    pub maximum_bit_depth: Option<u32>,
    #[serde(default)]
    pub tracks: Option<TracksContainer>,
    /// Universal Product Code for the album
    pub upc: Option<String>,
    /// Editorial description/review of the album
    pub description: Option<String>,
    /// Album goodies (booklets, liner notes PDFs)
    #[serde(default)]
    pub goodies: Option<Vec<Goody>>,
    /// Editorial awards (Qobuzissime, Album of the Week, press accolades).
    #[serde(default)]
    pub awards: Option<Vec<AlbumAward>>,
    /// Parental advisory / explicit content marker.
    #[serde(default)]
    pub parental_warning: Option<bool>,
    /// Full artist contributor list including roles. The primary artist is
    /// duplicated here as `roles: ["main-artist"]`; non-main entries are
    /// the album's featured artists.
    #[serde(default)]
    pub artists: Option<Vec<AlbumArtist>>,
}

/// Album artist contributor entry (main artist + featured artists).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumArtist {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub roles: Option<Vec<String>>,
}

/// A downloadable extra bundled with an album (e.g. PDF booklet)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goody {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub url: String,
    /// Original (full-size) URL
    #[serde(default)]
    pub original_url: String,
    /// File format id (e.g. 21 for PDF)
    #[serde(default)]
    pub file_format_id: Option<u32>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracksContainer {
    pub items: Vec<Track>,
    pub total: u32,
}

/// Artist model
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Artist {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    pub image: Option<ImageSet>,
    #[serde(default)]
    pub albums_count: Option<u32>,
    /// Biography (available when fetching full artist details)
    #[serde(default)]
    pub biography: Option<ArtistBiography>,
    /// Albums (available when fetching with extra=albums)
    #[serde(default)]
    pub albums: Option<ArtistAlbums>,
    /// Tracks where this artist appears (extra=tracks_appears_on)
    #[serde(default)]
    pub tracks_appears_on: Option<TracksContainer>,
    /// Curated playlists for this artist (extra=playlists)
    #[serde(default)]
    pub playlists: Option<Vec<Playlist>>,
}

/// Artist biography content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistBiography {
    pub summary: Option<String>,
    pub content: Option<String>,
    pub source: Option<String>,
}

/// Artist albums container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistAlbums {
    pub items: Vec<Album>,
    pub total: u32,
    #[serde(default)]
    pub offset: u32,
    #[serde(default)]
    pub limit: u32,
}

/// Playlist model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub owner: PlaylistOwner,
    pub images: Option<Vec<String>>,
    #[serde(default)]
    pub tracks_count: u32,
    #[serde(default)]
    pub duration: u32,
    #[serde(default)]
    pub is_public: bool,
    #[serde(default)]
    pub tracks: Option<TracksContainer>,
    pub genres: Option<Vec<PlaylistGenre>>,
    pub images150: Option<Vec<String>>,
    pub images300: Option<Vec<String>>,
    pub slug: Option<String>,
    pub users_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaylistOwner {
    #[serde(default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistGenre {
    pub id: u64,
    pub name: String,
    pub slug: Option<String>,
}

// ============ Metadata Types ============

/// Label model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: u64,
    pub name: String,
}

/// Genre model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

// ============ Search Types ============

/// Search results container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub albums: Option<SearchResultsPage<Album>>,
    pub tracks: Option<SearchResultsPage<Track>>,
    pub artists: Option<SearchResultsPage<Artist>>,
    pub playlists: Option<SearchResultsPage<Playlist>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultsPage<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub offset: u32,
    pub limit: u32,
}

/// Favorites container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favorites {
    pub albums: Option<SearchResultsPage<Album>>,
    pub tracks: Option<SearchResultsPage<Track>>,
    pub artists: Option<SearchResultsPage<Artist>>,
}

// ============ Discover API Types ============

// ============ Artist Page Types (/artist/page) ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageArtistAward {
    pub id: u64,
    pub name: String,
    pub awarded_at: Option<String>,
}

/// Award attached to an album. Shape is intentionally lenient because
/// Qobuz uses three different embedded shapes across endpoints:
/// - `/discover/index` — {id: int, name, awarded_at: "YYYY-MM-DD"}
/// - `/album/get`      — LegacyAwardDto {awardId: string, name,
///   publicationId, publicationName, awardSlug, awardedAt: long, …}
/// - `/artist/page`    — PageArtistAward {id: int, name, awarded_at}
///
/// id is emitted as String downstream so the frontend has a single
/// type to carry into /award/page and /award/getAlbums. The `alias`
/// list covers the LegacyAwardDto field name the web app never sees
/// but the mobile API uses.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AlbumAward {
    #[serde(
        default,
        alias = "awardId",
        alias = "award_id",
        deserialize_with = "deserialize_award_id"
    )]
    pub id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(
        default,
        alias = "awardedAt",
        deserialize_with = "deserialize_award_awarded_at"
    )]
    pub awarded_at: Option<String>,
}

fn deserialize_award_id<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(match value {
        Some(serde_json::Value::String(s)) if !s.is_empty() => Some(s),
        Some(serde_json::Value::Number(n)) => Some(n.to_string()),
        _ => None,
    })
}

fn deserialize_award_awarded_at<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(match value {
        Some(serde_json::Value::String(s)) => Some(s),
        Some(serde_json::Value::Number(n)) => Some(n.to_string()),
        _ => None,
    })
}
