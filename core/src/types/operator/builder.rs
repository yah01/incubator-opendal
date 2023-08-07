// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::collections::HashMap;
use std::sync::Arc;

use once_cell::sync::Lazy;

use crate::layers::*;
use crate::raw::*;
use crate::*;

/// # Operator build API
///
/// Operator should be built via [`OperatorBuilder`]. We recommend to use [`Operator::new`] to get started:
///
/// ```
/// # use anyhow::Result;
/// use opendal::services::Fs;
/// use opendal::Operator;
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     // Create fs backend builder.
///     let mut builder = Fs::default();
///     // Set the root for fs, all operations will happen under this root.
///     //
///     // NOTE: the root must be absolute path.
///     builder.root("/tmp");
///
///     // Build an `Operator` to start operating the storage.
///     let op: Operator = Operator::new(builder)?.finish();
///
///     Ok(())
/// }
/// ```
impl Operator {
    /// Create a new operator with input builder.
    ///
    /// OpenDAL will call `builder.build()` internally, so we don't need
    /// to import `opendal::Builder` trait.
    ///
    /// # Examples
    ///
    /// Read more backend init examples in [examples](https://github.com/apache/incubator-opendal/tree/main/examples).
    ///
    /// ```
    /// # use anyhow::Result;
    /// use opendal::services::Fs;
    /// use opendal::Operator;
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     // Create fs backend builder.
    ///     let mut builder = Fs::default();
    ///     // Set the root for fs, all operations will happen under this root.
    ///     //
    ///     // NOTE: the root must be absolute path.
    ///     builder.root("/tmp");
    ///
    ///     // Build an `Operator` to start operating the storage.
    ///     let op: Operator = Operator::new(builder)?.finish();
    ///
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new<B: Builder>(mut ab: B) -> Result<OperatorBuilder<impl Accessor>> {
        let acc = ab.build()?;
        Ok(OperatorBuilder::new(acc))
    }

    /// Create a new operator from given map.
    ///
    /// # Notes
    ///
    /// from_map is using static dispatch layers which is zero cost. via_map is
    /// using dynamic dispatch layers which has a bit runtime overhead with an
    /// extra vtable lookup and unable to inline. But from_map requires generic
    /// type parameter which is not always easy to be used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// use std::collections::HashMap;
    ///
    /// use opendal::services::Fs;
    /// use opendal::Operator;
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let map = HashMap::from([
    ///         // Set the root for fs, all operations will happen under this root.
    ///         //
    ///         // NOTE: the root must be absolute path.
    ///         ("root".to_string(), "/tmp".to_string()),
    ///     ]);
    ///
    ///     // Build an `Operator` to start operating the storage.
    ///     let op: Operator = Operator::from_map::<Fs>(map)?.finish();
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_map<B: Builder>(
        map: HashMap<String, String>,
    ) -> Result<OperatorBuilder<impl Accessor>> {
        let acc = B::from_map(map).build()?;
        Ok(OperatorBuilder::new(acc))
    }

    /// Create a new operator from given scheme and map.
    ///
    /// # Notes
    ///
    /// from_map is using static dispatch layers which is zero cost. via_map is
    /// using dynamic dispatch layers which has a bit runtime overhead with an
    /// extra vtable lookup and unable to inline. But from_map requires generic
    /// type parameter which is not always easy to be used.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// use std::collections::HashMap;
    ///
    /// use opendal::Operator;
    /// use opendal::Scheme;
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let map = HashMap::from([
    ///         // Set the root for fs, all operations will happen under this root.
    ///         //
    ///         // NOTE: the root must be absolute path.
    ///         ("root".to_string(), "/tmp".to_string()),
    ///     ]);
    ///
    ///     // Build an `Operator` to start operating the storage.
    ///     let op: Operator = Operator::via_map(Scheme::Fs, map)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn via_map(scheme: Scheme, map: HashMap<String, String>) -> Result<Operator> {
        let op = match scheme {
            #[cfg(feature = "services-azblob")]
            Scheme::Azblob => Self::from_map::<services::Azblob>(map)?.finish(),
            #[cfg(feature = "services-azdfs")]
            Scheme::Azdfs => Self::from_map::<services::Azdfs>(map)?.finish(),
            #[cfg(feature = "services-cacache")]
            Scheme::Cacache => Self::from_map::<services::Cacache>(map)?.finish(),
            #[cfg(feature = "services-cos")]
            Scheme::Cos => Self::from_map::<services::Cos>(map)?.finish(),
            #[cfg(feature = "services-dashmap")]
            Scheme::Dashmap => Self::from_map::<services::Dashmap>(map)?.finish(),
            #[cfg(feature = "services-etcd")]
            Scheme::Etcd => Self::from_map::<services::Etcd>(map)?.finish(),
            #[cfg(feature = "services-foundationdb")]
            Scheme::Foundationdb => Self::from_map::<services::Foundationdb>(map)?.finish(),
            #[cfg(feature = "services-fs")]
            Scheme::Fs => Self::from_map::<services::Fs>(map)?.finish(),
            #[cfg(feature = "services-ftp")]
            Scheme::Ftp => Self::from_map::<services::Ftp>(map)?.finish(),
            #[cfg(feature = "services-gcs")]
            Scheme::Gcs => Self::from_map::<services::Gcs>(map)?.finish(),
            #[cfg(feature = "services-ghac")]
            Scheme::Ghac => Self::from_map::<services::Ghac>(map)?.finish(),
            #[cfg(feature = "services-hdfs")]
            Scheme::Hdfs => Self::from_map::<services::Hdfs>(map)?.finish(),
            #[cfg(feature = "services-http")]
            Scheme::Http => Self::from_map::<services::Http>(map)?.finish(),
            #[cfg(feature = "services-ipfs")]
            Scheme::Ipfs => Self::from_map::<services::Ipfs>(map)?.finish(),
            #[cfg(feature = "services-ipmfs")]
            Scheme::Ipmfs => Self::from_map::<services::Ipmfs>(map)?.finish(),
            #[cfg(feature = "services-memcached")]
            Scheme::Memcached => Self::from_map::<services::Memcached>(map)?.finish(),
            #[cfg(feature = "services-memory")]
            Scheme::Memory => Self::from_map::<services::Memory>(map)?.finish(),
            #[cfg(feature = "services-mini-moka")]
            Scheme::MiniMoka => Self::from_map::<services::MiniMoka>(map)?.finish(),
            #[cfg(feature = "services-moka")]
            Scheme::Moka => Self::from_map::<services::Moka>(map)?.finish(),
            #[cfg(feature = "services-obs")]
            Scheme::Obs => Self::from_map::<services::Obs>(map)?.finish(),
            #[cfg(feature = "services-onedrive")]
            Scheme::Onedrive => Self::from_map::<services::Onedrive>(map)?.finish(),
            #[cfg(feature = "services-gdrive")]
            Scheme::Gdrive => Self::from_map::<services::Gdrive>(map)?.finish(),
            #[cfg(feature = "services-oss")]
            Scheme::Oss => Self::from_map::<services::Oss>(map)?.finish(),
            #[cfg(feature = "services-persy")]
            Scheme::Persy => Self::from_map::<services::Persy>(map)?.finish(),
            #[cfg(feature = "services-redis")]
            Scheme::Redis => Self::from_map::<services::Redis>(map)?.finish(),
            #[cfg(feature = "services-rocksdb")]
            Scheme::Rocksdb => Self::from_map::<services::Rocksdb>(map)?.finish(),
            #[cfg(feature = "services-s3")]
            Scheme::S3 => Self::from_map::<services::S3>(map)?.finish(),
            #[cfg(feature = "services-sftp")]
            Scheme::Sftp => Self::from_map::<services::Sftp>(map)?.finish(),
            #[cfg(feature = "services-sled")]
            Scheme::Sled => Self::from_map::<services::Sled>(map)?.finish(),
            #[cfg(feature = "services-supabase")]
            Scheme::Supabase => Self::from_map::<services::Supabase>(map)?.finish(),
            #[cfg(feature = "services-vercel-artifacts")]
            Scheme::VercelArtifacts => Self::from_map::<services::VercelArtifacts>(map)?.finish(),
            #[cfg(feature = "services-wasabi")]
            Scheme::Wasabi => Self::from_map::<services::Wasabi>(map)?.finish(),
            #[cfg(feature = "services-webdav")]
            Scheme::Webdav => Self::from_map::<services::Webdav>(map)?.finish(),
            #[cfg(feature = "services-webhdfs")]
            Scheme::Webhdfs => Self::from_map::<services::Webhdfs>(map)?.finish(),
            #[cfg(feature = "services-redb")]
            Scheme::Redb => Self::from_map::<services::Redb>(map)?.finish(),
            v => {
                return Err(Error::new(
                    ErrorKind::Unsupported,
                    "scheme is not enabled or supported",
                )
                .with_context("scheme", v))
            }
        };

        Ok(op)
    }

    /// Create a new layer with dynamic dispatch.
    ///
    /// # Notes
    ///
    /// `OperatorBuilder::layer()` is using static dispatch which is zero
    /// cost. `Operator::layer()` is using dynamic dispatch which has a
    /// bit runtime overhead with an extra vtable lookup and unable to
    /// inline.
    ///
    /// It's always recommended to use `OperatorBuilder::layer()` instead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::sync::Arc;
    /// # use anyhow::Result;
    /// use opendal::layers::LoggingLayer;
    /// use opendal::services::Fs;
    /// use opendal::Operator;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let op = Operator::new(Fs::default())?.finish();
    /// let op = op.layer(LoggingLayer::default());
    /// // All operations will go through the new_layer
    /// let _ = op.read("test_file").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn layer<L: Layer<FusedAccessor>>(self, layer: L) -> Self {
        Self::from_inner(Arc::new(
            TypeEraseLayer.layer(layer.layer(self.into_inner())),
        ))
    }
}

/// OperatorBuilder is a typed builder to build an Operator.
///
/// # Notes
///
/// OpenDAL uses static dispatch internally and only performs dynamic
/// dispatch at the outmost type erase layer. OperatorBuilder is the only
/// public API provided by OpenDAL come with generic parameters.
///
/// It's required to call `finish` after the operator built.
///
/// # Examples
///
/// For users who want to support many services, we can build a helper function like the following:
///
/// ```
/// use std::collections::HashMap;
///
/// use opendal::layers::LoggingLayer;
/// use opendal::layers::RetryLayer;
/// use opendal::services;
/// use opendal::Builder;
/// use opendal::Operator;
/// use opendal::Result;
/// use opendal::Scheme;
///
/// fn init_service<B: Builder>(cfg: HashMap<String, String>) -> Result<Operator> {
///     let op = Operator::from_map::<B>(cfg)?
///         .layer(LoggingLayer::default())
///         .layer(RetryLayer::new())
///         .finish();
///
///     Ok(op)
/// }
///
/// async fn init(scheme: Scheme, cfg: HashMap<String, String>) -> Result<()> {
///     let _ = match scheme {
///         Scheme::S3 => init_service::<services::S3>(cfg)?,
///         Scheme::Fs => init_service::<services::Fs>(cfg)?,
///         _ => todo!(),
///     };
///
///     Ok(())
/// }
/// ```
pub struct OperatorBuilder<A: Accessor> {
    accessor: A,
}

static RUNTIME: Lazy<tokio::runtime::Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

impl<A: Accessor> OperatorBuilder<A> {
    /// Create a new operator builder.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(accessor: A) -> OperatorBuilder<impl Accessor> {
        // Make sure error context layer has been attached.
        OperatorBuilder { accessor }
            .layer(ErrorContextLayer)
            .layer(CompleteLayer)
    }

    /// Create a new layer with static dispatch.
    ///
    /// # Notes
    ///
    /// `OperatorBuilder::layer()` is using static dispatch which is zero
    /// cost. `Operator::layer()` is using dynamic dispatch which has a
    /// bit runtime overhead with an extra vtable lookup and unable to
    /// inline.
    ///
    /// It's always recommended to use `OperatorBuilder::layer()` instead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::sync::Arc;
    /// # use anyhow::Result;
    /// use opendal::layers::LoggingLayer;
    /// use opendal::services::Fs;
    /// use opendal::Operator;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let op = Operator::new(Fs::default())?
    ///     .layer(LoggingLayer::default())
    ///     .finish();
    /// // All operations will go through the new_layer
    /// let _ = op.read("test_file").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn layer<L: Layer<A>>(self, layer: L) -> OperatorBuilder<L::LayeredAccessor> {
        OperatorBuilder {
            accessor: layer.layer(self.accessor),
        }
    }

    /// Finish the building to construct an Operator.
    pub fn finish(self) -> Operator {
        let ob = self.layer(TypeEraseLayer);
        let runtime =
            tokio::runtime::Handle::try_current().unwrap_or_else(|_| RUNTIME.handle().clone());
        let _guard = runtime.enter();

        Operator::from_inner(Arc::new(ob.accessor) as FusedAccessor)
            .layer(BlockingLayer::create().unwrap())
            .layer(LoggingLayer::default())
    }
}
