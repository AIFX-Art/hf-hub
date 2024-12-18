use crate::RepoType;

use super::{Api, ApiError, ApiRepo, ReqwestBadResponse};

/// todo docs
#[derive(Debug)]
pub enum RepoInfo {
    /// Model Variant
    Model(ModelInfo),
    // TODO add dataset and space info
}

impl RepoInfo {
    /// todo docs
    pub fn sha(&self) -> Option<&str> {
        match self {
            RepoInfo::Model(m) => m.sha.as_deref(),
        }
    }
}

impl From<ModelInfo> for RepoInfo {
    fn from(value: ModelInfo) -> Self {
        Self::Model(value)
    }
}

impl ApiRepo {
    /// Get the info object for a given repo.
    pub async fn repo_info(&self) -> Result<RepoInfo, ApiError> {
        match self.repo.repo_type {
            RepoType::Model => Ok(self
                .api
                .model_info(&self.repo.repo_id, Some(&self.repo.revision))
                .await?
                .into()),
            RepoType::Dataset => todo!(),
            RepoType::Space => todo!(),
        }
    }
}

impl Api {
    /// Get info on one specific model on huggingface.co
    ///
    /// Model can be private if you pass an acceptable token or are logged in.
    ///
    /// Args:
    ///     repo_id (`str`):
    ///         A namespace (user or an organization) and a repo name separated
    ///         by a `/`.
    ///     revision (`str`, *optional*):
    ///         The revision of the model repository from which to get the
    ///         information.
    ///     timeout (`float`, *optional*):
    ///         Whether to set a timeout for the request to the Hub.
    ///     securityStatus (`bool`, *optional*):
    ///         Whether to retrieve the security status from the model
    ///         repository as well.
    ///     files_metadata (`bool`, *optional*):
    ///         Whether or not to retrieve metadata for files in the repository
    ///         (size, LFS metadata, etc). Defaults to `False`.
    ///     expand (`List[ExpandModelProperty_T]`, *optional*):
    ///         List properties to return in the response. When used, only the properties in the list will be returned.
    ///         This parameter cannot be used if `securityStatus` or `files_metadata` are passed.
    ///         Possible values are `"author"`, `"baseModels"`, `"cardData"`, `"childrenModelCount"`, `"config"`, `"createdAt"`, `"disabled"`, `"downloads"`, `"downloadsAllTime"`, `"gated"`, `"gguf"`, `"inference"`, `"lastModified"`, `"library_name"`, `"likes"`, `"mask_token"`, `"model-index"`, `"pipeline_tag"`, `"private"`, `"safetensors"`, `"sha"`, `"siblings"`, `"spaces"`, `"tags"`, `"transformersInfo"`, `"trendingScore"` and `"widgetData"`.
    ///     token (Union[bool, str, None], optional):
    ///         A valid user access token (string). Defaults to the locally saved
    ///         token, which is the recommended method for authentication (see
    ///         https://huggingface.co/docs/huggingface_hub/quick-start#authentication).
    ///         To disable authentication, pass `False`.
    async fn model_info(
        &self,
        repo_id: &str,
        revision: Option<&str>,
    ) -> Result<ModelInfo, ApiError> {
        let url = if let Some(revision) = revision {
            format!(
                "{}/api/models/{repo_id}/revision/{}",
                self.endpoint,
                urlencoding::encode(revision)
            )
        } else {
            format!("{}/api/models/{repo_id}", self.endpoint)
        };

        // TODO add params for security status, blobs, expand, etc.

        let model_info: ModelInfo = self
            .client
            .get(url)
            .send()
            .await?
            .maybe_err()
            .await?
            .json()
            .await?;

        Ok(model_info)
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
/// todo docs
pub struct ModelInfo {
    /// todo docs
    #[serde(default)]    
    pub _id: Option<String>,

    /// todo docs
    #[serde(default)]
    #[serde(alias = "modelId")]    
    pub model_id: Option<String>,

    /// todo docs
    pub id: String,

    /// todo docs
    #[serde(default)]
    pub author: Option<String>,

    /// todo docs
    #[serde(default)]
    pub sha: Option<String>,

    /// todo docs
    #[serde(default)]
    #[serde(alias = "createdAt", alias = "created_at")]
    pub created_at: Option<String>,

    /// todo docs
    #[serde(default)]
    #[serde(alias = "lastModified", alias = "last_modified")]
    pub last_modified: Option<String>,

    /// todo docs
    #[serde(default)]
    pub private: Option<bool>,

    /// todo docs
    #[serde(default)]
    pub disabled: Option<bool>,

    /// todo docs
    #[serde(default)]
    pub downloads: Option<i32>,

    /// todo docs
    #[serde(default)]
    #[serde(alias = "downloadsAllTime")]
    pub downloads_all_time: Option<i32>,

    /// todo docs
    #[serde(default)]
    pub gated: Option<GatedStatus>,

    /// todo docs
    #[serde(default)]
    pub gguf: Option<HashMap<String, serde_json::Value>>,

    /// todo docs
    #[serde(default)]
    pub inference: Option<InferenceStatus>,

    /// todo docs
    #[serde(default)]
    pub likes: Option<i32>,

    /// todo docs
    #[serde(default)]
    pub library_name: Option<String>,

    /// todo docs
    #[serde(default)]
    pub tags: Option<Vec<String>>,

    /// todo docs
    #[serde(default)]
    pub pipeline_tag: Option<String>,

    /// todo docs
    #[serde(default)]
    pub mask_token: Option<String>,

    /// todo docs
    #[serde(default)]
    #[serde(alias = "cardData", alias = "card_data")]
    pub card_data: Option<ModelCardData>,

    /// todo docs
    #[serde(default)]
    #[serde(alias = "widgetData")]
    pub widget_data: Option<serde_json::Value>,

    /// todo docs
    #[serde(default)]
    #[serde(alias = "model-index", alias = "model_index")]
    pub model_index: Option<HashMap<String, serde_json::Value>>,

    /// todo docs
    #[serde(default)]
    pub config: Option<HashMap<String, serde_json::Value>>,

    /// todo docs
    #[serde(default)]
    #[serde(alias = "transformersInfo", alias = "transformers_info")]
    pub transformers_info: Option<TransformersInfo>,

    /// todo docs
    #[serde(default)]
    #[serde(alias = "trendingScore")]
    pub trending_score: Option<i32>,

    /// todo docs
    #[serde(default)]
    pub siblings: Option<Vec<RepoSibling>>,

    /// todo docs
    #[serde(default)]
    pub spaces: Option<Vec<String>>,

    /// todo docs
    #[serde(default)]
    pub safetensors: Option<SafeTensorsInfo>,
}

/// todo docs
#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GatedStatus {
    /// todo docs
    Auto,
    /// todo docs
    Manual,
    /// todo docs
    False,
}

impl<'de> Deserialize<'de> for GatedStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct GatedStatusVisitor;

        impl<'de> serde::de::Visitor<'de> for GatedStatusVisitor {
            type Value = GatedStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string \"auto\", \"manual\", or boolean false")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if !value {
                    Ok(GatedStatus::False)
                } else {
                    Err(E::custom("expected false"))
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "auto" => Ok(GatedStatus::Auto),
                    "manual" => Ok(GatedStatus::Manual),
                    _ => Err(E::custom("invalid value")),
                }
            }
        }

        deserializer.deserialize_any(GatedStatusVisitor)
    }
}

/// todo docs
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InferenceStatus {
    /// todo docs
    Warm,
    /// todo docs
    Cold,
    /// todo docs
    Frozen,
}

/// todo docs
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoSibling {
    /// todo docs
    pub rfilename: String,

    /// todo docs
    #[serde(default)]
    pub size: Option<i64>,

    /// todo docs
    #[serde(alias = "blobId")]
    #[serde(default)]
    pub blob_id: Option<String>,

    /// todo docs
    #[serde(default)]
    pub lfs: Option<BlobLfsInfo>,
}

/// todo docs
#[derive(Debug, Serialize, Deserialize)]
pub struct BlobLfsInfo {
    /// todo docs
    pub size: i64,
    /// todo docs
    pub sha256: String,

    /// todo docs
    #[serde(alias = "pointerSize")]
    pub pointer_size: i64,
}

/// todo docs
#[derive(Debug, Serialize, Deserialize)]
pub struct SafeTensorsInfo {
    /// todo docs
    pub parameters: i64,
    /// todo docs
    pub total: i64,
}

/// todo docs
// Note: You'll need to implement ModelCardData and TransformersInfo structs separately
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelCardData {
    // Add fields as needed
}

/// todo docs
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformersInfo {
    // Add fields as needed
}
