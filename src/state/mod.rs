mod token;

pub(crate) use token::ListRowToken;
pub(crate) use token::OpenDialogToken;
pub(crate) use token::SettingToken;
pub(crate) use token::ErrorToken;

use crate::{config, file, optimize, update};

/// アプリケーションの状態
pub(crate) struct State {
    pub config: config::Config,
    pub files: file::OpenFiles,
    pub open_dialog_token: OpenDialogToken,
    pub setting_token: SettingToken,
    pub updated_token: update::UpdatedToken,
    pub error_token: ErrorToken,
}

pub(crate) struct Jobs {
    pub optimize_job: optimize::OptimizeJob,
    pub update_job: update::UpdateJob,
}

/// 設定タブ
#[derive(PartialEq)]
pub enum SettingTab {
    General,
    Optimize,
    Output,
    Concurrent,
    About,
}

impl std::fmt::Display for SettingTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingTab::General => write!(f, "General"),
            SettingTab::Optimize => write!(f, "Optimize"),
            SettingTab::Output => write!(f, "Output"),
            SettingTab::Concurrent => write!(f, "Concurrent"),
            SettingTab::About => write!(f, "About"),
        }
    }
}
