use crate::{
    account::keys::{EncryptionPubKey, NullifierPubKey},
    indexer::{IndexerState, indexed_account::IndexedAccountState},
};

pub mod memory;

/// Generic key-value store interface.  Abstracts over different storage backends selected by SDK
/// consumers.
#[async_trait::async_trait]
pub trait KvStore: Send + Sync {
    /// Get the value associated with the given key.
    async fn get(&self, key: &[u8]) -> Option<Vec<u8>>;

    /// Puts the value associated with the given key.
    async fn put(&self, key: &[u8], value: &[u8]) {
        self.batch_put(&[(key, value)]).await;
    }

    /// Puts multiple key-value pairs in a batch operation.
    ///
    /// The batch operation must be atomic, meaning either all kv pairs are
    /// written or none are written.
    async fn batch_put(&self, items: &[(&[u8], &[u8])]);

    /// Deletes the value associated with the given key.
    async fn delete(&self, key: &[u8]);
}

pub(crate) trait TintDatabase: KvStore {
    async fn set_indexer(&self, state: &IndexerState);
    async fn load_indexer(&self) -> Option<IndexerState>;

    async fn set_indexed_account(
        &self,
        nullifier_pub: NullifierPubKey,
        encryption_pub: EncryptionPubKey,
        state: &IndexedAccountState,
    );
    async fn load_indexed_account(
        &self,
        nullifier_pub: NullifierPubKey,
        encryption_pub: EncryptionPubKey,
    ) -> Option<IndexedAccountState>;
}

impl<T: ?Sized + KvStore> TintDatabase for T {
    async fn set_indexer(&self, state: &IndexerState) {
        #[allow(clippy::unwrap_used)]
        let serialized = postcard::to_stdvec(state).unwrap();
        self.put(b"indexer", &serialized).await;
    }

    async fn load_indexer(&self) -> Option<IndexerState> {
        let serialized = self.get(b"indexer").await?;
        #[allow(clippy::unwrap_used)]
        let state = postcard::from_bytes(&serialized).unwrap();
        Some(state)
    }

    async fn set_indexed_account(
        &self,
        nullifier_pub: NullifierPubKey,
        encryption_pub: EncryptionPubKey,
        state: &IndexedAccountState,
    ) {
        let key = indexed_account_key(nullifier_pub, encryption_pub);

        #[allow(clippy::unwrap_used)]
        let serialized = postcard::to_stdvec(state).unwrap();
        self.put(&key, &serialized).await;
    }

    async fn load_indexed_account(
        &self,
        nullifier_pub: NullifierPubKey,
        encryption_pub: EncryptionPubKey,
    ) -> Option<IndexedAccountState> {
        let key = indexed_account_key(nullifier_pub, encryption_pub);

        let serialized = self.get(&key).await?;
        #[allow(clippy::unwrap_used)]
        let state = postcard::from_bytes(&serialized).unwrap();
        Some(state)
    }
}

/// Derives the database key for an account's indexed state from its viewing
/// and nullifying identity.
fn indexed_account_key(
    nullifier_pub_key: NullifierPubKey,
    encryption_pub_key: EncryptionPubKey,
) -> Vec<u8> {
    #[allow(clippy::unwrap_used)]
    postcard::to_stdvec(&(nullifier_pub_key, encryption_pub_key)).unwrap()
}
