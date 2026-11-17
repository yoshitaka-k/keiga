use crate::{config, event, file, optimize, update, ui, sounds, state};
use crate::event::{open, key, click, input, button};
use crate::ui::{setting, modal};
use crate::ui::assets::{constants, fonts, svg};
use crate::ui::main::{top, middle, bottom};

/// レンダーを管理する構造体
pub struct Rendar {
    /// アプリケーションの状態
    state: state::State,

    /// ジョブ
    jobs: state::Jobs,

    // 処理予約されたアクション
    pending_actions: Vec<event::EventAction>,

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
            setting_token: state::SettingToken::new(),
            error_token: state::ErrorToken::new(),
            updated_token: update::UpdatedToken::new(),
            scroll_token: state::ScrollToken::new(),
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
        // 選択された行を解除はイベント・アクションの前
        self.state.scroll_token.selected = false;

        // 最適化結果を反映
        self.optimize_result();

        // 入力イベントを処理
        self.input_events(ui);

        // ファイル追加時に最適化を実行
        self.optimize_run();

        // ファイル一覧のアクションを処理
        self.process_pending_actions(ui);

        // 状態カラーを作成
        let status_color = ui::StatusColor::new(ui.ctx());

        // 上部ボタンを表示
        top::view(
            ui,
            &self.state.config,
            &mut self.pending_actions,
        );

        // 状態とかボタンを表示するタスクバーを表示
        bottom::view(
            ui,
            &status_color,
            &mut self.state.files,
            &mut self.jobs.optimize_job,
            &mut self.state.error_token,
        );

        // リスト行の高さと行数を取得
        let row_height = ui.text_style_height(&egui::TextStyle::Body)
            .max(constants::CHECK_ICON_SIZE) + ui::SEPARATOR_HEIGHT;

        // ファイル一覧を表示
        middle::view(
            ui,
            &self.state,
            &mut self.pending_actions,
            &status_color,
            row_height,
        );

        // 設定ウィンドウを表示
        if self.state.setting_token.open {
            setting::view(
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
        // ドラッグ&ドロップされたファイルを処理
        input::drop(ui, &mut self.pending_actions);

        // Command + O キーが押されたらファイルダイアログを開く
        input::command_open(ui, &mut self.pending_actions);

        // Command + Comma キーが押されたら設定ウィンドウを開く
        input::command_comma(ui, &mut self.pending_actions);

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

    /// 処理予約されたアクションを処理
    /// * `files` - ファイル一覧
    /// * `pending_actions` - 処理予約されたアクション
    fn process_pending_actions(
        &mut self,
        ui: &egui::Ui,
    ) {
        // 処理予約されたアクションを取り出して空にする
        let mut pending_actions = std::mem::take(&mut self.pending_actions);

        // 取り出されたアクションを処理
        for action in pending_actions.drain(..) {
            let result = match action {
                event::EventAction::FileOpen => {
                    open::file(&self.state.config, &mut self.state.files)
                }
                event::EventAction::FolderOpen => {
                    open::folder(&self.state.config, &mut self.state.files)
                }
                event::EventAction::DropOpen(files) => {
                    open::drop(
                        ui.ctx(),
                        &files,
                        &self.state.config,
                        &mut self.state.files,
                    )
                }
                event::EventAction::SettingOpen(pos) => {
                    button::setting_open(&mut self.state.setting_token, pos);
                    Ok(())
                }
                event::EventAction::Click { index } => {
                    click::click(&mut self.state.files, index);
                    Ok(())
                }
                event::EventAction::DoubleClick(path) => {
                    click::double_click(&path)
                }
                event::EventAction::Up => {
                    key::up(&mut self.state.files, &mut self.state.scroll_token);
                    Ok(())
                }
                event::EventAction::Down => {
                    key::down(&mut self.state.files, &mut self.state.scroll_token);
                    Ok(())
                }
                event::EventAction::Enter { path } => {
                    click::double_click(&path)
                }
                event::EventAction::Space { path } => {
                    key::space(&path)
                }
                event::EventAction::Backspace => {
                    key::backspace(&mut self.state.files, &mut self.jobs.optimize_job)
                }
            };

            if let Err(e) = result {
                log::error!("Error processing action: {}", e);
                self.state.error_token.open = true;
                self.state.error_token.value = Some(e);
            }

            // 再描画を要求
            ui.ctx().request_repaint();
        }
    }
}
