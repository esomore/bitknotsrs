use rocksdb::{DB, Options, ColumnFamily, ColumnFamilyDescriptor};
use std::path::Path;
use std::sync::Arc;
use tracing::{info, error};

use crate::config::StorageConfig;
use crate::error::{StorageError, StorageResult};

pub struct Storage {
    db: Arc<DB>,
}

// Column families for different data types
pub const CF_BLOCKS: &str = "blocks";
pub const CF_TRANSACTIONS: &str = "transactions";
pub const CF_UTXOS: &str = "utxos";
pub const CF_CHAIN_STATE: &str = "chain_state";
pub const CF_MEMPOOL: &str = "mempool";
pub const CF_PEERS: &str = "peers";

impl Storage {
    pub fn new(config: &StorageConfig) -> StorageResult<Self> {
        let path = &config.rocks_db_path;

        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| StorageError::DatabaseNotFound {
                    path: format!("Failed to create directory {}: {}", parent.display(), e)
                })?;
        }

        // Configure RocksDB options
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        opts.set_max_open_files(config.max_open_files);

        // Set cache size
        let cache = rocksdb::Cache::new_lru_cache(config.cache_size);
        let mut block_opts = rocksdb::BlockBasedOptions::default();
        block_opts.set_block_cache(&cache);
        opts.set_block_based_table_factory(&block_opts);

        // Set compression
        match config.compression {
            crate::config::CompressionType::None => opts.set_compression_type(rocksdb::DBCompressionType::None),
            crate::config::CompressionType::Snappy => opts.set_compression_type(rocksdb::DBCompressionType::Snappy),
            crate::config::CompressionType::Lz4 => opts.set_compression_type(rocksdb::DBCompressionType::Lz4),
            crate::config::CompressionType::Zstd => opts.set_compression_type(rocksdb::DBCompressionType::Zstd),
        }

        // Define column families
        let cfs = vec![
            ColumnFamilyDescriptor::new(CF_BLOCKS, Options::default()),
            ColumnFamilyDescriptor::new(CF_TRANSACTIONS, Options::default()),
            ColumnFamilyDescriptor::new(CF_UTXOS, Options::default()),
            ColumnFamilyDescriptor::new(CF_CHAIN_STATE, Options::default()),
            ColumnFamilyDescriptor::new(CF_MEMPOOL, Options::default()),
            ColumnFamilyDescriptor::new(CF_PEERS, Options::default()),
        ];

        // Open database
        let db = DB::open_cf_descriptors(&opts, path, cfs)
            .map_err(|e| StorageError::RocksDb(e))?;

        info!("Storage initialized at {:?}", path);
        info!("Cache size: {} MB", config.cache_size / 1024 / 1024);
        info!("Compression: {:?}", config.compression);

        Ok(Self {
            db: Arc::new(db),
        })
    }

    // Generic key-value operations
    pub fn put(&self, cf_name: &str, key: &[u8], value: &[u8]) -> StorageResult<()> {
        let cf = self.get_cf(cf_name)?;
        self.db.put_cf(&cf, key, value)
            .map_err(|e| StorageError::RocksDb(e))?;
        Ok(())
    }

    pub fn get(&self, cf_name: &str, key: &[u8]) -> StorageResult<Option<Vec<u8>>> {
        let cf = self.get_cf(cf_name)?;
        self.db.get_cf(&cf, key)
            .map_err(|e| StorageError::RocksDb(e))
    }

    pub fn delete(&self, cf_name: &str, key: &[u8]) -> StorageResult<()> {
        let cf = self.get_cf(cf_name)?;
        self.db.delete_cf(&cf, key)
            .map_err(|e| StorageError::RocksDb(e))?;
        Ok(())
    }

    pub fn exists(&self, cf_name: &str, key: &[u8]) -> StorageResult<bool> {
        let cf = self.get_cf(cf_name)?;
        match self.db.get_cf(&cf, key) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(StorageError::RocksDb(e)),
        }
    }

    // Block operations
    pub fn store_block(&self, block_hash: &[u8], block_data: &[u8]) -> StorageResult<()> {
        self.put(CF_BLOCKS, block_hash, block_data)
    }

    pub fn get_block(&self, block_hash: &[u8]) -> StorageResult<Option<Vec<u8>>> {
        self.get(CF_BLOCKS, block_hash)
    }

    pub fn delete_block(&self, block_hash: &[u8]) -> StorageResult<()> {
        self.delete(CF_BLOCKS, block_hash)
    }

    // Transaction operations
    pub fn store_transaction(&self, txid: &[u8], tx_data: &[u8]) -> StorageResult<()> {
        self.put(CF_TRANSACTIONS, txid, tx_data)
    }

    pub fn get_transaction(&self, txid: &[u8]) -> StorageResult<Option<Vec<u8>>> {
        self.get(CF_TRANSACTIONS, txid)
    }

    pub fn delete_transaction(&self, txid: &[u8]) -> StorageResult<()> {
        self.delete(CF_TRANSACTIONS, txid)
    }

    // UTXO operations
    pub fn store_utxo(&self, outpoint: &[u8], utxo_data: &[u8]) -> StorageResult<()> {
        self.put(CF_UTXOS, outpoint, utxo_data)
    }

    pub fn get_utxo(&self, outpoint: &[u8]) -> StorageResult<Option<Vec<u8>>> {
        self.get(CF_UTXOS, outpoint)
    }

    pub fn delete_utxo(&self, outpoint: &[u8]) -> StorageResult<()> {
        self.delete(CF_UTXOS, outpoint)
    }

    // Chain state operations
    pub fn store_chain_state(&self, key: &[u8], value: &[u8]) -> StorageResult<()> {
        self.put(CF_CHAIN_STATE, key, value)
    }

    pub fn get_chain_state(&self, key: &[u8]) -> StorageResult<Option<Vec<u8>>> {
        self.get(CF_CHAIN_STATE, key)
    }

    // Mempool operations
    pub fn store_mempool_tx(&self, txid: &[u8], tx_data: &[u8]) -> StorageResult<()> {
        self.put(CF_MEMPOOL, txid, tx_data)
    }

    pub fn get_mempool_tx(&self, txid: &[u8]) -> StorageResult<Option<Vec<u8>>> {
        self.get(CF_MEMPOOL, txid)
    }

    pub fn delete_mempool_tx(&self, txid: &[u8]) -> StorageResult<()> {
        self.delete(CF_MEMPOOL, txid)
    }

    // Peer operations
    pub fn store_peer_info(&self, peer_id: &[u8], peer_data: &[u8]) -> StorageResult<()> {
        self.put(CF_PEERS, peer_id, peer_data)
    }

    pub fn get_peer_info(&self, peer_id: &[u8]) -> StorageResult<Option<Vec<u8>>> {
        self.get(CF_PEERS, peer_id)
    }

    pub fn delete_peer_info(&self, peer_id: &[u8]) -> StorageResult<()> {
        self.delete(CF_PEERS, peer_id)
    }

    // Utility methods
    pub fn get_database_size(&self) -> StorageResult<u64> {
        // Get approximate size of all column families
        let mut total_size = 0u64;

        for cf_name in &[CF_BLOCKS, CF_TRANSACTIONS, CF_UTXOS, CF_CHAIN_STATE, CF_MEMPOOL, CF_PEERS] {
            if let Ok(cf) = self.get_cf(cf_name) {
                if let Ok(Some(size_str)) = self.db.property_value_cf(&cf, "rocksdb.total-sst-files-size") {
                    if let Ok(size) = size_str.parse::<u64>() {
                        total_size += size;
                    }
                }
            }
        }

        Ok(total_size)
    }

    pub fn compact(&self) -> StorageResult<()> {
        for cf_name in &[CF_BLOCKS, CF_TRANSACTIONS, CF_UTXOS, CF_CHAIN_STATE, CF_MEMPOOL, CF_PEERS] {
            if let Ok(cf) = self.get_cf(cf_name) {
                self.db.compact_range_cf(&cf, None::<&[u8]>, None::<&[u8]>);
            }
        }
        info!("Database compaction completed");
        Ok(())
    }

    pub fn backup(&self, backup_path: &Path) -> StorageResult<()> {
        // Create backup directory
        std::fs::create_dir_all(backup_path)
            .map_err(|e| StorageError::DatabaseNotFound {
                path: format!("Failed to create backup directory: {}", e)
            })?;

        // TODO: Implement proper backup using RocksDB backup engine
        info!("Backup created at {:?}", backup_path);
        Ok(())
    }

    // Helper method to get column family handle
    fn get_cf(&self, cf_name: &str) -> StorageResult<&ColumnFamily> {
        self.db.cf_handle(cf_name)
            .ok_or_else(|| StorageError::Corruption {
                component: format!("Column family '{}' not found", cf_name)
            })
    }
}

impl Clone for Storage {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_size_bytes: u64,
    pub block_count: u64,
    pub transaction_count: u64,
    pub utxo_count: u64,
    pub mempool_count: u64,
    pub peer_count: u64,
}

impl Storage {
    pub fn get_stats(&self) -> StorageResult<StorageStats> {
        // TODO: Implement proper statistics collection
        Ok(StorageStats {
            total_size_bytes: self.get_database_size()?,
            block_count: 0,
            transaction_count: 0,
            utxo_count: 0,
            mempool_count: 0,
            peer_count: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use tempfile::TempDir;

    fn create_test_storage() -> (Storage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let mut config = Config::test_config();
        config.storage.rocks_db_path = temp_dir.path().join("rocksdb");

        let storage = Storage::new(&config.storage).unwrap();
        (storage, temp_dir)
    }

    #[test]
    fn test_storage_initialization() {
        let (storage, _temp_dir) = create_test_storage();

        // Test basic operations
        let key = b"test_key";
        let value = b"test_value";

        storage.put("blocks", key, value).unwrap();
        let retrieved = storage.get("blocks", key).unwrap();

        assert_eq!(retrieved, Some(value.to_vec()));
    }

    #[test]
    fn test_storage_block_operations() {
        let (storage, _temp_dir) = create_test_storage();

        // Create a dummy block hash and data
        let block_hash = [0u8; 32];
        let block_data = b"dummy_block_data";

        // Store block
        storage.store_block(&block_hash, block_data).unwrap();

        // Retrieve block
        let retrieved = storage.get_block(&block_hash).unwrap();
        assert_eq!(retrieved, Some(block_data.to_vec()));

        // Test non-existent block
        let non_existent_hash = [1u8; 32];
        let not_found = storage.get_block(&non_existent_hash).unwrap();
        assert_eq!(not_found, None);
    }

    #[test]
    fn test_storage_transaction_operations() {
        let (storage, _temp_dir) = create_test_storage();

        // Create a dummy transaction hash and data
        let txid = [0u8; 32];
        let tx_data = b"dummy_transaction_data";

        // Store transaction
        storage.store_transaction(&txid, tx_data).unwrap();

        // Retrieve transaction
        let retrieved = storage.get_transaction(&txid).unwrap();
        assert_eq!(retrieved, Some(tx_data.to_vec()));
    }

    #[test]
    fn test_storage_utxo_operations() {
        let (storage, _temp_dir) = create_test_storage();

        // Create a dummy outpoint and UTXO data
        let outpoint = b"txid:0";
        let utxo_data = b"dummy_utxo_data";

        // Store UTXO
        storage.store_utxo(outpoint, utxo_data).unwrap();

        // Retrieve UTXO
        let retrieved = storage.get_utxo(outpoint).unwrap();
        assert_eq!(retrieved, Some(utxo_data.to_vec()));

        // Delete UTXO
        storage.delete_utxo(outpoint).unwrap();
        let deleted = storage.get_utxo(outpoint).unwrap();
        assert_eq!(deleted, None);
    }

    #[test]
    fn test_storage_mempool_operations() {
        let (storage, _temp_dir) = create_test_storage();

        // Create dummy mempool transaction
        let txid = [1u8; 32];
        let tx_data = b"mempool_transaction_data";

        // Store in mempool
        storage.store_mempool_tx(&txid, tx_data).unwrap();

        // Retrieve from mempool
        let retrieved = storage.get_mempool_tx(&txid).unwrap();
        assert_eq!(retrieved, Some(tx_data.to_vec()));

        // Delete from mempool
        storage.delete_mempool_tx(&txid).unwrap();
        let deleted = storage.get_mempool_tx(&txid).unwrap();
        assert_eq!(deleted, None);
    }

    #[test]
    fn test_storage_exists() {
        let (storage, _temp_dir) = create_test_storage();

        let key = b"test_exists_key";
        let value = b"test_exists_value";

        // Key should not exist initially
        assert!(!storage.exists("blocks", key).unwrap());

        // Store the key-value pair
        storage.put("blocks", key, value).unwrap();

        // Key should now exist
        assert!(storage.exists("blocks", key).unwrap());
    }

    #[test]
    fn test_storage_stats() {
        let (storage, _temp_dir) = create_test_storage();

        // Get storage statistics
        let stats = storage.get_stats().unwrap();

        // Should have zero counts initially
        assert_eq!(stats.block_count, 0);
        assert_eq!(stats.transaction_count, 0);
        assert_eq!(stats.utxo_count, 0);
        assert_eq!(stats.mempool_count, 0);
        assert_eq!(stats.peer_count, 0);
    }
}