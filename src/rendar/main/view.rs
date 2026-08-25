use crate::app;
use crate::app::{UpdateJob, UpdatedToken};
use crate::file;
use crate::event::{open, drop};
use crate::optimize::OptimizeJob;
use crate::rendar::{self, SettingTab, ListRowToken, ErrorToken, SettingToken};
use crate::rendar::assets::{fonts, svg, SoundPlayer};
use crate::rendar::main::{top, list, bottom};
use crate::rendar::setting::view as setting_window;
use crate::rendar::modal;

/// レンダーを管理する構造体
pub struct Rendar {
    app: app::App,
    files: file::OpenFiles,

    // ファイルダイアログを開くタイミング
    open_dialog: bool,

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
            open_dialog: false,
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
}

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
        // 最適化結果を反映
        self.optimize_result();

        // 結果を反映後、まだ未処理があれば最適化を実行
        self.optimize_run();

        // スタイルを設定
        ui.ctx().global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;

        });

        // 開くボタンが押されてたらファイルダイアログを開く
        if self.open_dialog {
            self.open_dialog = false;
            if let Err(e) = open::open_files(
                &self.app,
                &mut self.files,
            ) {
                eprintln!("Error opening files: {}", e);
                self.error_token.value = Some(e);
                self.error_token.open = true;
            }
        }

        // ドラッグ&ドロップされたファイルを処理
        ui.ctx().input(|input| {
            let files = input.raw.dropped_files.clone();
            if let Err(e) = drop::drop_files(
                &files,
                &self.app,
                &mut self.files,
            ) {
                eprintln!("Error dropping files: {}", e);
                self.error_token.value = Some(e);
                self.error_token.open = true;
            }
        });

        // ファイル追加時に最適化を実行
        self.optimize_run();

        // パネルのスタイルを設定
        let top_panel_style = rendar::panel_style(ui, rendar::TOP_PANEL_INNER_MARGIN);
        let bottom_panel_style = rendar::panel_style(ui, rendar::BOTTOM_PANEL_INNER_MARGIN);

        // 上部ボタンを表示
        egui::Panel::top("top_taskbar").frame(top_panel_style).show(ui, |ui| {
            top::view(ui, &mut self.files, &mut self.open_dialog, &mut self.setting_token);
        });

        // 状態とかボタンを表示するタスクバーを表示
        egui::Panel::bottom("bottom_taskbar").frame(bottom_panel_style).show(ui, |ui| {
            bottom::view(ui, &mut self.files, &mut self.optimize_job, &mut self.error_token);
        });

        // 中央パネルを表示
        egui::CentralPanel::default().show(ui, |ui| {
            let row_height = list::row_height(ui);
            let total_rows = self.files.paths().len();

            // ファイル一覧を表示
            // リスト行をクリックしたら選択状態を保持
            let row_clicked = egui::ScrollArea::vertical()
                // コンテナが小さい時に縮小させない
                .auto_shrink([false; 2])
                // スクロールビューの高さを指定
                .max_height(ui.available_height())
                // コンテナ内の表示
                .show_rows(ui, row_height, total_rows, |ui, row_range| {
                    // 表示する行の情報
                    let list_row_token = ListRowToken {
                        range: row_range,
                        height: row_height,
                    };

                    // ファイル一覧を表示
                    list::view(ui, &mut self.files, &mut self.optimize_job, list_row_token, &mut self.error_token)
                }).inner;

            // リスト行以外をクリックしたら選択解除
            if ui.input(|i| i.pointer.primary_clicked()) && !row_clicked {
                self.files.set_selected_id(None);
            }
        });

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
