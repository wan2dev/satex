use serde::{Deserialize, Serialize};

///
/// 配置文件监控扫描间隔时间
///
const DEFAULT_REFRESH_FREQUENCY_MILLIS: u64 = 5000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refresh {
    ///
    /// 是否开启刷新
    ///
    #[serde(default)]
    pub enabled: bool,

    ///
    /// 刷新频率
    ///
    pub frequency: u64,
}

impl Default for Refresh {
    fn default() -> Self {
        Self {
            enabled: true,
            frequency: DEFAULT_REFRESH_FREQUENCY_MILLIS,
        }
    }
}
