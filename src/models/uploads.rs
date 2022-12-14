use std::path::PathBuf;

use actix_multipart_extract::{File, MultipartForm};
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};
use utoipa::{IntoParams, ToSchema};

use crate::internal::file::can_have_thumbnail;

use crate::database::entity::uploads;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UploadData {
    pub id: String,
    pub uploader: String,
    pub name: String,
    pub original_name: String,
    pub url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub hash: String,
    pub size: i64,
    pub album_id: Option<String>,
    pub public: bool,
    #[schema(value_type = f64)]
    pub uploaded: DateTime<Utc>,
}

impl From<uploads::Model> for UploadData {
    fn from(upload: uploads::Model) -> Self {
        Self {
            id: upload.id,
            uploader: upload.uploader,
            name: upload.name,
            original_name: upload.original_name,
            hash: upload.hash,
            uploaded: upload.uploaded.into(),
            size: upload.size,
            album_id: upload.album_id,
            public: upload.public,
            // These fields are not stored in database
            // They are filled in by the route returning it
            url: None,
            thumbnail_url: None,
        }
    }
}

impl UploadData {
    /// Computes and sets the URL based on a root storage path
    pub fn set_url(&mut self, mut root_path: PathBuf) {
        root_path.push(&self.name);
        self.url = Some(root_path.as_path().display().to_string().replace("\\", "/"))
    }

    /// Computes and sets the URL based on root storage path
    /// This will only set if a valid image or extension was sent
    pub fn set_thumbnail_url(&mut self, mut root_path: PathBuf) {
        if can_have_thumbnail(&self.name) {
            root_path.push(format!("thumb/{}", &self.name));
            self.thumbnail_url = Some(root_path.as_path().display().to_string().replace("\\", "/"));
        }
    }
}

/// File stats for user.
#[derive(Serialize, ToSchema)]
pub struct UploadStats {
    /// Total usage in bytes
    pub usage: i64,
}

/// Delete multiple files.
#[derive(Deserialize, ToSchema)]
pub struct BatchDeleteRequest {
    /// IDs to delete.
    pub ids: Vec<String>,
}

/// Response containing information about deleted files.
#[derive(Serialize, ToSchema, Default)]
pub struct BatchDeleteResponse {
    /// All successfully deleted files.
    pub deleted: Vec<String>,

    /// Errors for all failed deletions.
    pub errors: Vec<BatchFileError>,
}

/// Error for an individual item in a batch operation.
#[derive(Serialize, ToSchema)]
pub struct BatchFileError {
    /// ID of the item.
    pub id: String,

    // Error while executing operation.
    pub error: String,
}

/// Identical file was already uploaded.
#[derive(Serialize, ToSchema)]
pub struct UploadConflict {
    pub message: String,
    pub upload: UploadData,
}

/// Upload a file.
#[derive(Deserialize, MultipartForm, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadFile {
    #[schema(value_type = String, format = Binary)]
    pub upload_file: File,
}

#[derive(Deserialize, IntoParams)]
pub struct UploadQuery {
    /// Query by name of file.
    pub query: Option<String>,
    /// For non admins, this must be a public album
    /// or a private album owned by you.
    pub album_id: Option<String>,
    /// If accessing another user as a non admin, this must be `true`
    pub public: Option<bool>,
}
