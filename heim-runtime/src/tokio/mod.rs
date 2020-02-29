pub use tokio::{join, pin, try_join};

pub mod task {
    #[inline]
    pub async fn spawn_blocking<F, T>(f: F) -> Result<T, crate::task::JoinError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        tokio::task::spawn_blocking(f).await.map_err(|e| {
            if e.is_cancelled() {
                return crate::task::JoinError::Cancel;
            }
            if e.is_panic() {
                return crate::task::JoinError::Panic;
            }

            crate::task::JoinError::Other
        })
    }
}

pub mod fs {
    use std::io;
    use std::path::Path;

    use tokio::stream::Stream;

    pub use tokio::io::{AsyncBufReadExt as _, BufReader};
    // Re-exports
    pub use tokio::fs::{read, read_dir, read_link, read_to_string};

    pub async fn path_exists<T>(path: T) -> bool
    where
        T: AsRef<Path>,
    {
        match tokio::fs::metadata(path).await {
            Ok(..) => true,
            Err(..) => false,
        }
    }

    pub async fn read_lines<T>(path: T) -> io::Result<impl Stream<Item = io::Result<String>>>
    where
        T: AsRef<Path>,
    {
        let file = tokio::fs::File::open(path).await?;
        let reader = BufReader::new(file);
        Ok(reader.lines())
    }
}
