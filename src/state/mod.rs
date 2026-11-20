mod token;

pub(crate) use token::SettingToken;
pub(crate) use token::ErrorToken;
pub(crate) use token::ScrollToken;

use crate::{config, lang, file, optimize, update};

/// アプリケーションの状態
pub(crate) struct State {
    pub config: config::Config,
    pub files: file::OpenFiles,
    pub setting_token: SettingToken,
    pub updated_token: update::UpdatedToken,
    pub error_token: ErrorToken,
    pub scroll_token: ScrollToken,
}

/// ジョブ
pub(crate) struct Jobs {
    pub scan_job: file::ScanJob,
    pub meta_job: file::MetaJob,
    pub optimize_job: optimize::OptimizeJob,
    pub update_job: update::UpdateJob,
}

/// 設定タブ
#[derive(PartialEq)]
pub(crate) enum SettingTab {
    General,
    Optimize,
    Output,
    Concurrent,
    About,
}
impl std::fmt::Display for SettingTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingTab::General => write!(f, "{}", lang::current().setting.general),
            SettingTab::Optimize => write!(f, "{}", lang::current().setting.optimize),
            SettingTab::Output => write!(f, "{}", lang::current().setting.output),
            SettingTab::Concurrent => write!(f, "{}", lang::current().setting.concurrent),
            SettingTab::About => write!(f, "{}", lang::current().setting.about),
        }
    }
}
