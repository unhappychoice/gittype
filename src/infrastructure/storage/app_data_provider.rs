use crate::Result;
use std::path::PathBuf;

pub trait AppDataProvider {
    fn get_app_data_dir() -> Result<PathBuf> {
        #[cfg(not(feature = "test-mocks"))]
        {
            use crate::GitTypeError;
            let data_dir = if cfg!(debug_assertions) {
                std::env::current_dir().map_err(|e| {
                    GitTypeError::ExtractionFailed(format!(
                        "Could not get current directory: {}",
                        e
                    ))
                })?
            } else {
                let home_dir = dirs::home_dir().ok_or_else(|| {
                    GitTypeError::ExtractionFailed("Could not determine home directory".to_string())
                })?;
                home_dir.join(".gittype")
            };

            std::fs::create_dir_all(&data_dir)?;
            Ok(data_dir)
        }

        #[cfg(feature = "test-mocks")]
        {
            Ok(PathBuf::from("/tmp/test"))
        }
    }
}
