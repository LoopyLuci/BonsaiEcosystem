//! Error types for bias detection and confidence extraction

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error type for bias detection and confidence operations
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
pub enum BiasDetectorError {
    /// Pattern matching failed
    #[error("Pattern matching failed: {reason}")]
    PatternMatchingFailed { reason: String },

    /// Classifier inference failed
    #[error("Classifier inference failed: {reason}")]
    ClassifierInferenceFailed { reason: String },

    /// Confidence extraction failed
    #[error("Confidence extraction failed: {reason}")]
    ConfidenceExtractionFailed { reason: String },

    /// Confidence calibration validation failed
    #[error("Confidence calibration validation failed: {reason}")]
    CalibrationValidationFailed { reason: String },

    /// Self-consistency sampling failed
    #[error("Self-consistency sampling failed: {reason}")]
    SelfConsistencySamplingFailed { reason: String },

    /// Invalid configuration
    #[error("Invalid configuration: {reason}")]
    InvalidConfiguration { reason: String },

    /// Serialization error
    #[error("Serialization failed: {reason}")]
    SerializationFailed { reason: String },

    /// Internal error
    #[error("Internal error: {reason}")]
    Internal { reason: String },
}

impl BiasDetectorError {
    /// Create a PatternMatchingFailed error
    pub fn pattern_matching_failed(reason: impl Into<String>) -> Self {
        Self::PatternMatchingFailed {
            reason: reason.into(),
        }
    }

    /// Create a ClassifierInferenceFailed error
    pub fn classifier_inference_failed(reason: impl Into<String>) -> Self {
        Self::ClassifierInferenceFailed {
            reason: reason.into(),
        }
    }

    /// Create a ConfidenceExtractionFailed error
    pub fn confidence_extraction_failed(reason: impl Into<String>) -> Self {
        Self::ConfidenceExtractionFailed {
            reason: reason.into(),
        }
    }

    /// Create a CalibrationValidationFailed error
    pub fn calibration_validation_failed(reason: impl Into<String>) -> Self {
        Self::CalibrationValidationFailed {
            reason: reason.into(),
        }
    }

    /// Create a SelfConsistencySamplingFailed error
    pub fn self_consistency_sampling_failed(reason: impl Into<String>) -> Self {
        Self::SelfConsistencySamplingFailed {
            reason: reason.into(),
        }
    }

    /// Create an InvalidConfiguration error
    pub fn invalid_configuration(reason: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            reason: reason.into(),
        }
    }

    /// Create a SerializationFailed error
    pub fn serialization_failed(reason: impl Into<String>) -> Self {
        Self::SerializationFailed {
            reason: reason.into(),
        }
    }

    /// Create an Internal error
    pub fn internal(reason: impl Into<String>) -> Self {
        Self::Internal {
            reason: reason.into(),
        }
    }
}

impl From<serde_json::Error> for BiasDetectorError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationFailed {
            reason: err.to_string(),
        }
    }
}

impl From<regex::Error> for BiasDetectorError {
    fn from(err: regex::Error) -> Self {
        Self::PatternMatchingFailed {
            reason: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_matching_failed() {
        let err = BiasDetectorError::pattern_matching_failed("regex error");
        assert!(matches!(err, BiasDetectorError::PatternMatchingFailed { .. }));
    }

    #[test]
    fn test_error_display() {
        let err = BiasDetectorError::classifier_inference_failed("model error");
        let display = format!("{}", err);
        assert!(display.contains("Classifier inference failed"));
    }

    #[test]
    fn test_serde_json_error_conversion() {
        let json_err: BiasDetectorError = serde_json::from_str::<serde_json::Value>("invalid")
            .map(|_| ())
            .unwrap_err()
            .into();
        assert!(matches!(
            json_err,
            BiasDetectorError::SerializationFailed { .. }
        ));
    }

    #[test]
    fn test_regex_error_conversion() {
        let regex_err = regex::Regex::new("(?P<invalid");
        let ahf_err: BiasDetectorError = regex_err.unwrap_err().into();
        assert!(matches!(
            ahf_err,
            BiasDetectorError::PatternMatchingFailed { .. }
        ));
    }
}
