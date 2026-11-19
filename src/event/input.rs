use crate::{file, event};

/// ドロップされたファイルを処理する
/// * `ui` - ウィジェットのUI
/// * `pending_actions` - 処理予約されたアクション
pub fn drop(
    ui: &egui::Ui,
    pending_actions: &mut Vec<event::EventAction>,
) {
    let files: Vec<std::path::PathBuf> = ui.ctx().input(|input| {
        input.raw.dropped_files.iter().map(|f| f.path().to_path_buf()).collect()
    });

    if files.is_empty() { return; }

    pending_actions.push(event::EventAction::DropOpen(files));
}

/// Command + O キーが押されたらファイルダイアログを開く
/// * `ui` - ウィジェットのUI
pub fn command_open(ui: &mut egui::Ui, pending_actions: &mut Vec<event::EventAction>) {
    #[cfg(target_os = "macos")]
    {
        // Command + O キーが押されたらフォルダダイアログを開く
        if ui.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::COMMAND)
            && input.key_pressed(egui::Key::O)
        }) {
            pending_actions.push(event::EventAction::FolderOpen);
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // Command + O キーが押されたらファイルダイアログを開く
        if ui.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::COMMAND)
            && input.key_pressed(egui::Key::O)
        }) {
            pending_actions.push(event::EventAction::FileOpen);
        }

        // Ctrl + Shift + O キーが押されたらフォルダダイアログを開く
        if ui.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::COMMAND | egui::Modifiers::SHIFT)
            && input.key_pressed(egui::Key::O)
        }) {
            pending_actions.push(event::EventAction::FolderOpen);
        }
    }
}

/// Command + Comma キーが押されたら設定ウィンドウを開く
/// * `ui` - ウィジェットのUI
pub fn command_comma(ui: &mut egui::Ui, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::Comma)
    }) {
        let pos = ui.ctx().input(|input| {
            input.viewport().outer_rect.map(|rect| rect.min)
        });
        pending_actions.push(event::EventAction::SettingOpen(pos));
    }
}

/// Command + W キーが押されたら設定ウィンドウを閉じる
/// * `ui` - ウィジェットのUI
pub fn command_w(ui: &mut egui::Ui, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| {
        input.modifiers.matches_exact(egui::Modifiers::COMMAND)
        && input.key_pressed(egui::Key::W)
    }) {
        pending_actions.push(event::EventAction::SettingClose);
    }
}

/// 上矢印キーが押されたら選択された行を上に移動する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn arrow_up(ui: &mut egui::Ui, files: &mut file::OpenFiles, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::ArrowUp))
        && files.selected_index().is_some() {
            pending_actions.push(event::EventAction::Up);
        }
}

/// 下矢印キーが押されたら選択された行を下に移動する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn arrow_down(ui: &mut egui::Ui, files: &mut file::OpenFiles, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::ArrowDown))
        && files.selected_index().is_some() {
            pending_actions.push(event::EventAction::Down);
        }
}

/// エンターキーが押されたら選択された行を開く
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn enter(ui: &mut egui::Ui, files: &mut file::OpenFiles, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::Enter))
        && let Some(image_file) = files.selected_image() {
            pending_actions.push(event::EventAction::Enter {
                path: image_file.reveal_path().clone(),
            });
        }
}

/// スペースキーが押されたら選択された行を開く
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn space(ui: &mut egui::Ui, files: &mut file::OpenFiles, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::Space))
        && let Some(image_file) = files.selected_image() {
            pending_actions.push(event::EventAction::Space {
                path: image_file.reveal_path().clone(),
            });
        }
}

/// バックスペースキーが押されたら選択された行を削除する
/// * `ui` - ウィジェットのUI
/// * `files` - 開いているファイル
/// * `pending_actions` - 処理予約
pub fn backspace(ui: &mut egui::Ui, files: &mut file::OpenFiles, pending_actions: &mut Vec<event::EventAction>) {
    if ui.input(|input| input.key_pressed(egui::Key::Backspace))
        && files.selected_index().is_some() {
            pending_actions.push(event::EventAction::Backspace);
        }
}
