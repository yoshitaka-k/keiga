use crate::app::{self, UpdateJob, UpdatedToken};
use crate::file;
use crate::event::{open, drop, key_up, click, button};
use crate::optimize::OptimizeJob;
use crate::rendar::{self, StatusColor, SettingTab, ListRowToken, ErrorToken, SettingToken, OpenDialogToken};
use crate::rendar::assets::{constants, fonts, svg, SoundPlayer};
use crate::rendar::main::{self, top, list, bottom};
use crate::rendar::setting::view as setting_window;
use crate::rendar::modal;

/// レンダーを管理する構造体
pub struct Rendar {
    app: app::App,
    files: file::OpenFiles,
    status_color: StatusColor,

    // スクロールしている描画範囲
    vertical_scroll_offset: Option<std::ops::Range<usize>>,

    // キーイベント時のスクロール位置
    pending_scroll_y: Option<f32>,

    // ファイルダイアログを開くタイミング
    open_dialog_token: OpenDialogToken,

    // 設定ウィンドウのトークン
    setting_token: SettingToken,

    // エラーモーダルを表示するためのトークン
    error_token: ErrorToken,

    // 更新モーダルを表示するためのトークン
    updated_token: UpdatedToken,

    // 最適化ジョブ
    optimize_job: OptimizeJob,

    // 更新ジョブ
    update_job: UpdateJob,

    // 効果音プレイヤー
    sound: SoundPlayer,
}

/// Rendar の実装
impl Rendar {
    /// 新しい Rendar を作成
    /// * `cc` - 作成コンテキスト
    /// * `app` - アプリケーション
    /// * `return` - Rendar のインスタンス
    pub fn new(cc: &eframe::CreationContext<'_>, app: app::App) -> Self {
        // フォントと SVG ローダーを追加
        fonts::install(&cc.egui_ctx);
        svg::install(&cc.egui_ctx);

        // 前回保存した App があれば復元（なければ引数の app を使う）
        let app = cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or(app);

        // 開くファイルのインスタンスを作成
        let mut files = file::OpenFiles::new();

        // 拡張子を設定
        files.set_extensions(app.extensions_to_string());

        Self {
            app,
            files,
            status_color: StatusColor::new(&cc.egui_ctx),
            vertical_scroll_offset: None,
            pending_scroll_y: None,
            open_dialog_token: OpenDialogToken {
                file_dialog: false,
                folder_dialog: false,
            },
            setting_token: SettingToken {
                open: false,
                pos: None,
                tab: SettingTab::Concurrent,
            },
            error_token: ErrorToken {
                open: false,
                value: None,
            },
            updated_token: UpdatedToken {
                open: false,
                check: None,
            },
            optimize_job: OptimizeJob::new(cc.egui_ctx.clone()),
            update_job: UpdateJob::new(cc.egui_ctx.clone()),
            sound: SoundPlayer::new(),
        }
    }

    /// ファイルを最適化実行
    fn optimize_run(&mut self) {
        self.optimize_job.run(&self.app, &mut self.files);
    }

    /// 最適化結果を反映する
    fn optimize_result(&mut self) {
        // 最適化前か最適化中かどうかをチェック
        let was_busy = self.files.has_standby() || self.files.has_optimizing();

        // 最適化結果を反映
        self.optimize_job.result(&mut self.files);

        // 最適化後か最適化中が終わっているかどうかをチェック
        let now_idle = !self.files.has_standby() && !self.files.has_optimizing();

        // 効果音を再生
        if *self.app.play_sound() && was_busy && now_idle {
            // 効果音の音量を設定
            self.sound.set_volume(*self.app.sound_volume());

            if self.files.has_error() {
                self.sound.play_alert();
            } else {
                self.sound.play_completed();
            }
        }
    }

    /// リスト行の高さを取得
    /// * `ui` - UI
    /// * `return` - 行高
    fn row_height(&self, ui: &egui::Ui) -> f32 {
        ui.text_style_height(&egui::TextStyle::Body).max(constants::CHECK_ICON_SIZE) + main::SEPARATOR_HEIGHT
    }

    /// 選択行が見える範囲の外なら、スクロール位置を調整
    /// * `limit_id` - 制限 ID
    /// * `list_viewport_height` - 利用可能な高さ
    /// * `row_stride` - 行のストライド（行の高さ + 行の間隔）
    fn ensure_selected_row_visible(&mut self, list_viewport_height: f32, row_stride: f32, item_spacing: f32) {
        // 選択されている行のインデックスを取得
        let Some(index) = self.files.get_selected_index() else {
            return;
        };

        // スクロールしている描画範囲を取得
        let Some(range) = self.vertical_scroll_offset.clone() else {
            return;
        };

        // 最後の行はスクロールしないようにする
        let end = if range.end > 1 { range.end - 1 } else { range.end };
        // println!("index: {}, range.start: {}, range.end: {}, end: {}", index, range.start, range.end, end);

        // 選択されている行が見える範囲の外なら、スクロール位置を調整
        if index <= range.start {
            self.pending_scroll_y = Some(index as f32 * row_stride);
        } else if index >= end {
            let content_height = (index + 1) as f32 * row_stride - item_spacing;
            if content_height > list_viewport_height {
                self.pending_scroll_y = Some(content_height - list_viewport_height);
            }
        }
    }

    /// ダイアログを開く
    fn open_dialog(&mut self) {
        // フォルダダイアログを開くボタンが押されてたらフォルダダイアログを開く
        if self.open_dialog_token.folder_dialog {
            self.open_dialog_token.folder_dialog = false;
            if let Err(e) = open::folder(
                &self.app,
                &mut self.files,
            ) {
                eprintln!("Error opening folders: {}", e);
                self.error_token.open = true;
                self.error_token.value = Some(e);
            }
        }

        // ファイルダイアログを開くボタンが押されてたらファイルダイアログを開く
        if self.open_dialog_token.file_dialog {
            self.open_dialog_token.file_dialog = false;
            if let Err(e) = open::file(
                &self.app,
                &mut self.files,
            ) {
                eprintln!("Error opening files: {}", e);
                self.error_token.open = true;
                self.error_token.value = Some(e);
            }
        }
    }

    /// ドラッグ&ドロップされたファイルを処理
    /// * `ui` - UI
    fn drop_files(&mut self, ui: &egui::Ui) {
        ui.ctx().input(|input| {
            let files = input.raw.dropped_files.clone();
            if let Err(e) = drop::drop_files(
                &files,
                &self.app,
                &mut self.files,
            ) {
                eprintln!("Error dropping files: {}", e);
                self.error_token.open = true;
                self.error_token.value = Some(e);
            }
        });
    }
}

/// eframe::App の実装
impl eframe::App for Rendar {
    /// 終了前に App の状態を保存
    /// * `storage` - ストレージ
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.app);
    }

    /// ユーザーインターフェースを描画
    /// * `ui` - ユーザーインターフェース
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // スタイルを設定
        ui.ctx().global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
        });

        // イベント処理のためのアクションを保持
        let mut pending_actions: Vec<main::EventAction> = Vec::new();

        #[cfg(target_os = "macos")]
        {
            // Command + O キーが押されたらフォルダダイアログを開く
            if ui.input(|input| {
                input.modifiers.matches_exact(egui::Modifiers::COMMAND)
                && input.key_released(egui::Key::O)
            }) {
                button::folder_open(ui, &mut self.open_dialog_token);
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Command + O キーが押されたらファイルダイアログを開く
            if ui.input(|input| {
                input.modifiers.matches_exact(egui::Modifiers::COMMAND)
                && input.key_released(egui::Key::O)
            }) {
                button::file_open(ui, &mut self.open_dialog_token);
            }

            // Ctrl + Shift + O キーが押されたらフォルダダイアログを開く
            if ui.input(|input| {
                input.modifiers.matches_exact(egui::Modifiers::COMMAND | egui::Modifiers::SHIFT)
                && input.key_released(egui::Key::O)
            }) {
                button::folder_open(ui, &mut self.open_dialog_token);
            }
        }

        // Command + Comma キーが押されたら設定ウィンドウを開く
        if ui.input(|input| {
            input.modifiers.matches_exact(egui::Modifiers::COMMAND) && input.key_released(egui::Key::Comma)
        }) {
            button::setting_open(ui, &mut self.setting_token);
        }

        // 上キーが押されたら処理予約
        if ui.input(|input| input.key_released(egui::Key::ArrowUp)) {
            if let Some(image_file) = self.files.selected_image_file() {
                pending_actions.push(main::EventAction::Up {
                    id: *image_file.id(),
                });
            }
        }

        // 下キーが押されたら処理予約
        if ui.input(|input| input.key_released(egui::Key::ArrowDown)) {
            if let Some(image_file) = self.files.selected_image_file() {
            pending_actions.push(main::EventAction::Down {
                id: *image_file.id(),
            });
        }
        }

        // 削除キーが押されたら処理予約
        if ui.input(|input| input.key_released(egui::Key::Backspace)) {
            if self.files.selected_id().is_some() {
                pending_actions.push(main::EventAction::Backspace);
            }
        }

        // エンターキーが押されたら処理予約
        if ui.input(|input| input.key_released(egui::Key::Enter)) {
            let Some(image_file) = self.files.selected_image_file() else {
                return;
            };
            pending_actions.push(main::EventAction::Enter {
                path: image_file.reveal_path().clone(),
            });
        }

        // スペースキーが押されたら処理予約
        if ui.input(|input| input.key_released(egui::Key::Space)) {
            let Some(image_file) = self.files.selected_image_file() else {
                return;
            };
            pending_actions.push(main::EventAction::Space {
                path: image_file.reveal_path().clone(),
            });
        }

        // 最適化結果を反映
        self.optimize_result();

        // 結果を反映後、まだ未処理があれば最適化を実行
        self.optimize_run();

        // ダイアログを開く
        self.open_dialog();

        // ドラッグ&ドロップされたファイルを処理
        self.drop_files(ui);

        // ファイル追加時に最適化を実行
        self.optimize_run();

        // パネルのスタイルを設定
        let top_panel_style = rendar::panel_style(ui, rendar::TOP_PANEL_INNER_MARGIN);
        let bottom_panel_style = rendar::panel_style(ui, rendar::BOTTOM_PANEL_INNER_MARGIN);

        // リスト行の高さと行数を取得
        let row_height = self.row_height(ui);
        let total_rows = self.files.paths().len();

        // 利用可能な高さを設定
        let mut list_viewport_height: f32 = 0.0;
        // 行のストライド（行の高さ + 行の間隔）を設定
        let mut row_stride: f32 = 0.0;
        // 行の間隔を設定
        let mut item_spacing: f32 = 0.0;

        // 上部ボタンを表示
        egui::Panel::top("top_taskbar").frame(top_panel_style).show(ui, |ui| {
            top::view(ui, &mut self.open_dialog_token, &mut self.setting_token);
        });

        // 状態とかボタンを表示するタスクバーを表示
        egui::Panel::bottom("bottom_taskbar").frame(bottom_panel_style).show(ui, |ui| {
            bottom::view(ui, &self.status_color, &mut self.files, &mut self.optimize_job, &mut self.error_token);
        });

        // 中央パネルを表示
        egui::CentralPanel::default().show(ui, |ui| {
            // 利用可能な高さを設定
            list_viewport_height = ui.available_height();
            // 行のストライド（行の高さ + 行の間隔）を設定
            row_stride = row_height + ui.spacing().item_spacing.y;
            // 行の間隔を設定
            item_spacing = ui.spacing().item_spacing.y;

            // スクロールエリアを作成
            let mut scroll_area = egui::ScrollArea::vertical()
                // コンテナが小さい時に縮小させない
                .auto_shrink([false; 2])
                // スクロールビューの高さを指定
                .max_height(ui.available_height());

            // スクロール位置を設定
            if let Some(scroll_y) = self.pending_scroll_y.take() {
                scroll_area = scroll_area.vertical_scroll_offset(scroll_y);
            }

            // ファイル一覧を表示
            // リスト行をクリックしたら選択状態を保持
            let row_clicked = scroll_area.show_rows(
                ui, row_height, total_rows,
                |ui, row_range|
            {
                // 垂直スクロールオフセットを設定
                self.vertical_scroll_offset = Some(row_range.clone());

                // 表示する行の情報
                let list_row_token = ListRowToken {
                    range: row_range,
                    height: row_height,
                };

                // ファイル一覧を表示
                list::view(
                    ui,
                    &self.status_color,
                    &self.files,
                    list_row_token,
                    &mut pending_actions,
                )
            }).inner;

            // リスト行以外をクリックしたら選択解除
            if ui.input(|i| i.pointer.primary_clicked()) && !row_clicked {
                self.files.set_selected_id(None);
                self.vertical_scroll_offset = None;
                self.pending_scroll_y = None;
            }
        });

        // クリックアクションを処理
        for action in pending_actions {
            match action {
                main::EventAction::Click { id } => {
                    self.files.set_selected_id(Some(id));
                }
                main::EventAction::DoubleClick { path } => {
                    if let Err(e) = click::double_click(&path) {
                        eprintln!("Error revealing file: {}", e);
                        self.error_token.open = true;
                        self.error_token.value = Some(e);
                    }
                }
                main::EventAction::Up { id } => {
                    let min_id = self.files.get_min_id();

                    // 選択されているファイルの ID が最小の ID より大きい場合
                    if id > min_id {
                        self.files.set_selected_id(Some(id - 1));
                    } else {
                        self.files.set_selected_id(Some(min_id));
                    }

                    // 選択されている行が見える範囲の外なら、スクロール位置を調整
                    self.ensure_selected_row_visible(list_viewport_height, row_stride, item_spacing);
                }
                main::EventAction::Down { id } => {
                    let max_id = self.files.get_max_id();

                    // 選択されているファイルの ID が最大の ID より小さい場合
                    if id < max_id {
                        self.files.set_selected_id(Some(id + 1));
                    } else {
                        self.files.set_selected_id(Some(max_id));
                    }

                    // 選択されている行が見える範囲の外なら、スクロール位置を調整
                    self.ensure_selected_row_visible(list_viewport_height, row_stride, item_spacing);
                }
                main::EventAction::Enter { path } => {
                    if let Err(e) = click::double_click(&path) {
                        eprintln!("Error revealing file: {}", e);
                        self.error_token.open = true;
                        self.error_token.value = Some(e);
                    }
                }
                main::EventAction::Space { path } => {
                    if let Err(e) = key_up::space(&path) {
                        eprintln!("Error revealing file: {}", e);
                        self.error_token.open = true;
                        self.error_token.value = Some(e);
                    }
                }
                main::EventAction::Backspace => {
                    if let Err(e) = key_up::backspace(&mut self.files, &mut self.optimize_job) {
                        eprintln!("Error canceling file: {}", e);
                        self.error_token.open = true;
                        self.error_token.value = Some(e);
                    }
                }
            }
        }

        // 設定ウィンドウを表示
        if self.setting_token.open {
            setting_window::view(ui.ctx(), &mut self.app, &mut self.setting_token, &mut self.update_job, &mut self.updated_token);
        }

        // エラーモーダルを表示
        if self.error_token.open {
            modal::error(ui.ctx(), &mut self.error_token);
        }
    }
}
