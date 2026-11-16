use crate::{config, event, file, optimize, update, ui, sounds, state};
use crate::event::{open, key, click, input};
use crate::ui::modal;
use crate::ui::assets::{constants, fonts, svg};
use crate::ui::main::{top, middle, bottom};
use crate::ui::setting::view as setting_window;

/// レンダーを管理する構造体
pub struct Rendar {
    /// アプリケーションの状態
    state: state::State,

    /// ジョブ
    jobs: state::Jobs,

    // 処理予約されたアクション
    pending_actions: Vec<event::EventAction>,

    // スクロールしている描画範囲
    vertical_scroll_offset: Option<std::ops::Range<usize>>,

    // キーイベント時のスクロール位置
    pending_scroll_y: Option<f32>,

    // 効果音プレイヤー
    sound: sounds::SoundPlayer,
}

/// Rendar の実装
impl Rendar {
    /// 新しい Rendar を作成
    /// * `cc` - 作成コンテキスト
    /// * `config` - 設定
    /// * `return` - Rendar のインスタンス
    pub fn new(cc: &eframe::CreationContext<'_>, config: config::Config) -> Self {
        // フォントと SVG ローダーを追加
        fonts::install(&cc.egui_ctx);
        svg::install(&cc.egui_ctx);

        // スタイルを設定
        cc.egui_ctx.global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
        });

        // 前回保存した Config があれば復元（なければ引数の config を使う）
        let mut config = cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or(config);

        // 範囲外の値をクランプ
        config.clamp_values();

        // アプリケーションの状態を作成
        let state = state::State {
            config,
            files: file::OpenFiles::new(),
            open_dialog_token: state::OpenDialogToken {
                file_dialog: false,
                folder_dialog: false,
            },
            setting_token: state::SettingToken {
                open: false,
                pos: None,
                tab: state::SettingTab::Concurrent,
            },
            error_token: state::ErrorToken {
                open: false,
                value: None,
            },
            updated_token: update::UpdatedToken {
                open: false,
                check: None,
            },
        };

        // ジョブを作成
        let jobs = state::Jobs {
            optimize_job: optimize::OptimizeJob::new(cc.egui_ctx.clone()),
            update_job: update::UpdateJob::new(cc.egui_ctx.clone()),
        };

        // Rendar のインスタンスを作成
        Self {
            state,
            jobs,
            pending_actions: Vec::new(),
            vertical_scroll_offset: None,
            pending_scroll_y: None,
            sound: sounds::SoundPlayer::new(),
        }
    }
}

/// eframe::App の実装
impl eframe::App for Rendar {
    /// 終了前に Config の状態を保存
    /// * `storage` - ストレージ
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state.config);
    }

    /// ユーザーインターフェースを描画
    /// * `ui` - ユーザーインターフェース
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 入力イベントを処理
        self.input_events(ui);

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

        // 状態カラーを作成
        let status_color = ui::StatusColor::new(ui.ctx());

        // リスト行の高さと行数を取得
        let row_height = self.row_height(ui);
        let total_rows = self.state.files.len();

        // 利用可能な高さを設定
        let mut list_viewport_height: f32 = 0.0;
        // 行のストライド（行の高さ + 行の間隔）を設定
        let mut row_stride: f32 = 0.0;
        // 行の間隔を設定
        let mut item_spacing: f32 = 0.0;

        // 上部ボタンを表示
        top::view(
            ui,
            &self.state.config,
            &mut self.state.open_dialog_token,
            &mut self.state.setting_token,
        );

        // 状態とかボタンを表示するタスクバーを表示
        bottom::view(
            ui,
            &status_color,
            &mut self.state.files,
            &mut self.jobs.optimize_job,
            &mut self.state.error_token,
        );

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
                let list_row_token = state::ListRowToken {
                    range: row_range,
                    height: row_height,
                };

                // ファイル一覧を表示
                middle::view(
                    ui,
                    &status_color,
                    &self.state.files,
                    list_row_token,
                    &mut self.pending_actions,
                )
            }).inner;

            // リスト行以外をクリックしたら選択解除
            if ui.input(|i| i.pointer.primary_clicked()) && !row_clicked {
                self.state.files.set_selected_index(None);
                self.vertical_scroll_offset = None;
                self.pending_scroll_y = None;
            }
        });

        // ファイル一覧のアクションを処理
        self.process_pending_actions(
            list_viewport_height,
            row_stride,
            item_spacing,
        );

        // 設定ウィンドウを表示
        if self.state.setting_token.open {
            setting_window::view(
                ui.ctx(),
                &mut self.state.config,
                &mut self.state.setting_token,
                &mut self.jobs.update_job,
                &mut self.state.updated_token,
            );
        }

        // エラーモーダルを表示
        if self.state.error_token.open {
            modal::error(
                ui.ctx(),
                &mut self.state.error_token,
            );
        }
    }
}

/// ファイル一覧のアクションを処理
impl Rendar {
    /// 入力イベントを処理
    /// * `ui` - UI
    fn input_events(&mut self, ui: &mut egui::Ui) {
        // Command + O キーが押されたらファイルダイアログを開く
        input::command_open(ui, &mut self.state.open_dialog_token);

        // Command + Comma キーが押されたら設定ウィンドウを開く
        input::command_comma(ui, &mut self.state.setting_token);

        // 上キーが押されたら処理予約
        input::arrow_up(ui, &mut self.state.files, &mut self.pending_actions);

        // 下キーが押されたら処理予約
        input::arrow_down(ui, &mut self.state.files, &mut self.pending_actions);

        // エンターキーが押されたら処理予約
        input::enter(ui, &mut self.state.files, &mut self.pending_actions);

        // スペースキーが押されたら処理予約
        input::space(ui, &mut self.state.files, &mut self.pending_actions);

        // 削除キーが押されたら処理予約
        input::backspace(ui, &mut self.state.files, &mut self.pending_actions);
    }

    /// 最適化結果を反映する
    fn optimize_result(&mut self) {
        // 最適化前か最適化中かどうかをチェック
        let was_busy = self.state.files.has_standby() || self.state.files.has_optimizing();

        // 最適化結果を反映
        self.jobs.optimize_job.result(&mut self.state.files);

        // 最適化後か最適化中が終わっているかどうかをチェック
        let now_idle = !self.state.files.has_standby() && !self.state.files.has_optimizing();

        // 効果音を再生
        if *self.state.config.play_sound() && was_busy && now_idle {
            // 効果音の音量を設定
            self.sound.set_volume(*self.state.config.sound_volume());

            if self.state.files.has_error() {
                self.sound.play_alert();
            } else {
                self.sound.play_completed();
            }
        }
    }

    /// ファイルを最適化実行
    fn optimize_run(&mut self) {
        self.jobs.optimize_job.run(&self.state.config, &mut self.state.files);
    }

    /// リスト行の高さを取得
    /// * `ui` - UI
    /// * `return` - 行高
    fn row_height(&self, ui: &egui::Ui) -> f32 {
        ui.text_style_height(&egui::TextStyle::Body).max(constants::CHECK_ICON_SIZE) + ui::SEPARATOR_HEIGHT
    }

    /// 選択行が見える範囲の外なら、スクロール位置を調整
    /// * `list_viewport_height` - 利用可能な高さ
    /// * `row_stride` - 行のストライド（行の高さ + 行の間隔）
    fn ensure_selected_row_visible(&mut self, list_viewport_height: f32, row_stride: f32, item_spacing: f32) {
        // 選択されている行のインデックスを取得
        let Some(index) = self.state.files.selected_index() else {
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
        if *index <= range.start {
            self.pending_scroll_y = Some(*index as f32 * row_stride);
        } else if *index >= end {
            let content_height = (*index + 1) as f32 * row_stride - item_spacing;
            if content_height > list_viewport_height {
                self.pending_scroll_y = Some(content_height - list_viewport_height);
            }
        }
    }

    /// ダイアログを開く
    fn open_dialog(&mut self) {
        // フォルダダイアログを開くボタンが押されてたらフォルダダイアログを開く
        if self.state.open_dialog_token.folder_dialog {
            self.state.open_dialog_token.folder_dialog = false;
            if let Err(e) = open::folder(
                &self.state.config,
                &mut self.state.files,
            ) {
                log::error!("Error opening folders: {}", e);
                self.state.error_token.open = true;
                self.state.error_token.value = Some(e);
            }
        }

        // ファイルダイアログを開くボタンが押されてたらファイルダイアログを開く
        if self.state.open_dialog_token.file_dialog {
            self.state.open_dialog_token.file_dialog = false;
            if let Err(e) = open::file(
                &self.state.config,
                &mut self.state.files,
            ) {
                log::error!("Error opening files: {}", e);
                self.state.error_token.open = true;
                self.state.error_token.value = Some(e);
            }
        }
    }

    /// ドラッグ&ドロップされたファイルを処理
    /// * `ui` - UI
    fn drop_files(&mut self, ui: &egui::Ui) {
        if let Err(e) = input::drop(ui, &self.state.config, &mut self.state.files) {
            log::error!("Error dropping files: {}", e);
            self.state.error_token.open = true;
            self.state.error_token.value = Some(e);
        }
    }

    /// 処理予約されたアクションを処理
    /// * `list_viewport_height` - 利用可能な高さ
    /// * `row_stride` - 行のストライド（行の高さ + 行の間隔）
    /// * `item_spacing` - 行の間隔
    fn process_pending_actions(
        &mut self,
        list_viewport_height: f32,
        row_stride: f32,
        item_spacing: f32,
    ) {
        // 処理予約されたアクションを取り出して空にする
        let mut pending_actions = std::mem::take(&mut self.pending_actions);

        // 取り出されたアクションを処理
        for action in pending_actions.drain(..) {
            match action {
                event::EventAction::Click { index } => {
                    click::click(&mut self.state.files, index);
                }
                event::EventAction::DoubleClick { path } => {
                    if let Err(e) = click::double_click(&path) {
                        log::error!("Error revealing file: {}", e);
                        self.state.error_token.open = true;
                        self.state.error_token.value = Some(e);
                    }
                }
                event::EventAction::Up => {
                    key::up(&mut self.state.files);

                    // 選択されている行が見える範囲の外なら、スクロール位置を調整
                    self.ensure_selected_row_visible(list_viewport_height, row_stride, item_spacing);
                }
                event::EventAction::Down => {
                    key::down(&mut self.state.files);

                    // 選択されている行が見える範囲の外なら、スクロール位置を調整
                    self.ensure_selected_row_visible(list_viewport_height, row_stride, item_spacing);
                }
                event::EventAction::Enter { path } => {
                    if let Err(e) = click::double_click(&path) {
                        log::error!("Error revealing file: {}", e);
                        self.state.error_token.open = true;
                        self.state.error_token.value = Some(e);
                    }
                }
                event::EventAction::Space { path } => {
                    if let Err(e) = key::space(&path) {
                        log::error!("Error revealing file: {}", e);
                        self.state.error_token.open = true;
                        self.state.error_token.value = Some(e);
                    }
                }
                event::EventAction::Backspace => {
                    if let Err(e) = key::backspace(&mut self.state.files, &mut self.jobs.optimize_job) {
                        log::error!("Error canceling file: {}", e);
                        self.state.error_token.open = true;
                        self.state.error_token.value = Some(e);
                    }
                }
            }
        }
    }
}
