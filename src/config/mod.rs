use crate::config::refresh::Refresh;
use crate::config::router::Router;
use crate::config::server::Server;
use crate::config::tracing::Tracing;
use satex_core::Error;
use satex_core::component::Component;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod refresh;
pub mod router;
pub mod server;
pub mod tracing;

///
/// 配置文件
///
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
    ///
    /// 配置刷新频率
    ///
    #[serde(default)]
    pub refresh: Refresh,

    ///
    /// 服务配置
    ///
    #[serde(default)]
    pub server: Server,

    ///
    /// 路由配置
    ///
    #[serde(default)]
    pub router: Router,

    ///
    /// 服务解析配置
    ///
    #[serde(default)]
    pub resolvers: Vec<Component>,

    ///
    /// 日志配置
    ///
    #[serde(default)]
    pub tracing: Tracing,
}

impl Config {
    pub fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let bytes = std::fs::read(path).map_err(Error::new)?;
        serde_yaml::from_slice(&bytes).map_err(Error::new)
    }
}
