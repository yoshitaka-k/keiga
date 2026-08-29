use crate::{app, file, error};
use crate::event::{self, drop, button};
use crate::rendar::{OpenDialogToken, SettingToken};

/// ドロップされたファイルを処理する
/// * `ui` - ウィジェットのUI
/// * `app` - アプリケーション
/// * `open_files` - 開いているファイル
/// * return: エラーが発生した場合はエラーを返す
pub fn drop(ui: &egui::Ui, app: &app::App, open_files: &mut file::OpenFiles) -> error::Result<()> {
    ui.ctx().input(|input| {
        let files = input.raw.dropped_files.clone();
        drop::drop_files(
            &files,
            app,
            open_files,
        )
    })?;

    Ok(())
}

/// Command + O キーが押されたらファイルダイアログを開く
/// * `ui` - ウィジェットのUI
/// * `open_dialog_token` - ダイアログトークン
pub fn command_open(ui: &mut egui::Ui, open_dialog_token: &mut OpenDialogToken) {
    #[cfg(target_os = "macos")]
    {
        // Command + O キーが押されたらフォルダダイアログを開く
        if ui.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::COMMAND)
            && input.key_pressed(egui::Key::O)
        }) {
            button::folder_open(ui, open_dialog_token);
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Command + O キーが押されたらファイルダイアログを開く
        if ui.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::COMMAND)
            && input.key_pressed(egui::Key::O)
        }) {
            button::file_open(ui, open_dialog_token);
        }

        // Ctrl + Shift + O キーが押されたらフォルダダイアログを開く
        if ui.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::COMMAND | egui::Modifiers::SHIFT)
            && input.key_pressed(egui::Key::O)
        }) {
            button::folder_open(ui, open_dialog_token);
        }
    }
}

/// Command + Comma キーが押されたら設定ウィンドウを開く
/// * `ui` - ウィジェットのUI
/// * `setting_token` - 設定ウィンドウトークン
pub fn command_comma(ui: &mut egui::Ui, setting_token: &mut SettingToken) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::Comma)
    }) {
        button::setting_open(ui, setting_token);
    }
}

/// 上矢印キーが押されたら選択された行を上に移動する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn arrow_up(ui: &mut egui::Ui, files: &mut file::OpenFiles, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::ArrowUp)) {
        if let Some(image_file) = files.selected_image_file() {
            pending_actions.push(event::EventAction::Up {
                id: *image_file.id(),
            });
        }
    }
}

/// 下矢印キーが押されたら選択された行を下に移動する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn arrow_down(ui: &mut egui::Ui, files: &mut file::OpenFiles, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::ArrowDown)) {
        if let Some(image_file) = files.selected_image_file() {
            pending_actions.push(event::EventAction::Down {
                id: *image_file.id(),
            });
        }
    }
}

/// エンターキーが押されたら選択された行を開く
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn enter(ui: &mut egui::Ui, files: &mut file::OpenFiles, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
        if let Some(image_file) = files.selected_image_file() {
            pending_actions.push(event::EventAction::Enter {
                path: image_file.reveal_path().clone(),
            });
        }
    }
}

/// スペースキーが押されたら選択された行を開く
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn space(ui: &mut egui::Ui, files: &mut file::OpenFiles, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::Space)) {
        if let Some(image_file) = files.selected_image_file() {
            pending_actions.push(event::EventAction::Space {
                path: image_file.reveal_path().clone(),
            });
        }
    }
}

/// バックスペースキーが押されたら選択された行を削除する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn backspace(ui: &mut egui::Ui, files: &mut file::OpenFiles, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::Backspace)) {
        if files.selected_id().is_some() {
            pending_actions.push(event::EventAction::Backspace);
        }
    }
}
