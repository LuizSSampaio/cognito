use std::sync::Arc;

use item::Item;
use log::info;
use nucleo::{Config, Nucleo};
use walkdir::WalkDir;

pub mod item;

pub struct CognitoFinder {
    engine: Nucleo<Item>,
}

impl Default for CognitoFinder {
    fn default() -> Self {
        let config = Config::DEFAULT.match_paths();
        let notify = Arc::new(|| info!("Search results updated!"));
        let num_threads = None;
        let columns = 1;

        Self {
            engine: Nucleo::new(config, notify, num_threads, columns),
        }
    }
}

impl CognitoFinder {
    pub fn new<F>(notify_callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        let config = Config::DEFAULT.match_paths();
        let notify = Arc::new(notify_callback);
        let num_threads = None;
        let columns = 1;

        Self {
            engine: Nucleo::new(config, notify, num_threads, columns),
        }
    }

    pub fn index_directory<P: AsRef<std::path::Path>>(
        &mut self,
        root_path: P,
    ) -> anyhow::Result<usize> {
        let injector = self.engine.injector();
        let mut file_count = 0;

        for entry in WalkDir::new(root_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_string_lossy().to_string();
            let file_name = entry.file_name().to_string_lossy().to_string();

            let item = Item {
                title: file_name,
                path,
            };

            injector.push(item, |item, row| {
                row[0] = item.path.clone().into();
            });

            file_count += 1;
        }

        Ok(file_count)
    }

    pub fn search(&mut self, query: &str, max_results: usize) -> Vec<Item> {
        self.engine.pattern.reparse(
            0,
            query,
            nucleo::pattern::CaseMatching::Smart,
            nucleo::pattern::Normalization::Smart,
            false,
        );

        let _status = self.engine.tick(500);
        let snapshot = self.engine.snapshot();

        snapshot
            .matched_items(..)
            .take(max_results)
            .map(|item| item.data.clone())
            .collect()
    }

    pub fn search_status(&mut self) -> (bool, bool) {
        let status = self.engine.tick(0);
        (status.changed, !status.running)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_basic_file_search() {
        let temp_dir = std::env::temp_dir();

        fs::write(temp_dir.join("test_file.txt"), "content").unwrap();
        fs::write(temp_dir.join("another_file.rs"), "rust code").unwrap();
        fs::write(temp_dir.join("config.json"), "{}").unwrap();

        let mut finder = CognitoFinder::default();
        finder.index_directory(temp_dir).unwrap();

        let results = finder.search("test", 10);
        assert!(!results.is_empty());
        assert!(results.iter().any(|f| f.title.contains("test_file")));

        let results = finder.search("rs", 10);
        assert!(results.iter().any(|f| f.title.contains("another_file.rs")));
    }
}
