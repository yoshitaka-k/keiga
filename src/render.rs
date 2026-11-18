use crate::{config, event, file, optimize, update, ui, sounds, state, lang};
use crate::event::input;

/// レンダーを管理する構造体
pub struct Render {
    /// アプリケーションの状態
    state: state::State,

    /// ジョブ
    jobs: state::Jobs,

    // 処理予約されたアクション
    pending_actions: Vec<event::EventAction>,

    // 効果音プレイヤー
    sound: sounds::SoundPlayer,

    /// 最適化中か待機中かどうか
    was_busy: bool,
}

/// Render の実装
impl Render {
    /// 新しい Render を作成
    /// * `cc` - 作成コンテキスト
    /// * `config` - 設定
    /// * `return` - Render のインスタンス
    pub fn new(cc: &eframe::CreationContext<'_>, config: config::Config) -> Self {
        // フォントと SVG ローダーを追加
        ui::install(&cc.egui_ctx);

        // スタイルを設定
        cc.egui_ctx.global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
        });

        // 前回保存した Config があれば復元（なければ引数の config を使う）
        let mut config = cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or(config);

        // 言語を設定
        lang::set(config.language());

        // テーマを設定
        cc.egui_ctx.set_visuals(config.theme().visuals());

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

        // Render のインスタンスを作成
        Self {
            state,
            jobs,
            pending_actions: Vec::new(),
            sound: sounds::SoundPlayer::new(),
            was_busy: false,
        }
    }
}

/// eframe::App の実装
impl eframe::App for Render {
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
        self.jobs.optimize_job.result(&mut self.state.files);

        // 入力イベントを処理
        self.input_events(ui);

        // ファイル追加時に最適化を実行
        self.jobs.optimize_job.run(&self.state.config, &mut self.state.files);

        // ファイル一覧のアクションを処理
        self.process_pending_actions(ui);

        // ファイル情報を更新
        self.state.files.update_file_info();

        // 効果音を再生
        self.play_sound();

        // メインウィンドウを表示
        ui::draw(
            ui,
            &mut self.state,
            &mut self.jobs,
            &mut self.pending_actions,
        );
    }
}

/// ファイル一覧のアクションを処理
impl Render {
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

    /// 効果音を再生
    fn play_sound(&mut self) {
        // 最適化前か最適化中かどうか（忙しいかどうか）を取得
        let busy = self.is_busy();

        // ファイルがあって、忙しい状態が終わったら効果音を再生
        if !self.state.files.is_empty()
            && *self.state.config.play_sound()
            && self.was_busy && !busy
        {
            self.sound.set_volume(*self.state.config.sound_volume());
            if self.state.files.has_error() {
                self.sound.play_alert();
            } else {
                self.sound.play_completed();
            }
        }

        // 忙しいかどうかを更新
        self.was_busy = busy;
    }

    /// 処理予約されたアクションを処理
    /// * `ui` - UI
    fn process_pending_actions(&mut self, ui: &egui::Ui) {
        // 処理予約されたアクションを取り出して空にする
        let mut pending_actions = std::mem::take(&mut self.pending_actions);

        // 取り出されたアクションを処理
        for action in pending_actions.drain(..) {
            if let Err(e) = event::handle(ui.ctx(), &mut self.state, &mut self.jobs, action) {
                log::error!("Error processing action: {}", e);
                self.state.error_token.open = true;
                self.state.error_token.value = Some(e);
            }

            // 再描画を要求
            ui.ctx().request_repaint();
        }
    }

    /// 最適化中か待機中かどうかをチェック
    /// * `return` - 最適化中か待機中かどうか
    fn is_busy(&self) -> bool {
        self.state.files.has_standby()
            || self.state.files.has_optimizing()
    }
}
