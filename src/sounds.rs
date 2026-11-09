mod generated {
    use crate::sounds::SoundSource;
    include!(concat!(env!("OUT_DIR"), "/sounds_generated.rs"));
}

pub(crate) use generated::*;

use std::io::Cursor;
use rodio::{Decoder, MixerDeviceSink, Player};

/// 埋め込み効果音データ
pub(crate) struct SoundSource {
    pub bytes: &'static [u8],
}

impl SoundSource {
    /// 新しい効果音データを作成する
    pub const fn new(bytes: &'static [u8]) -> Self {
        Self { bytes }
    }
}

/// 効果音プレイヤー
pub(crate) struct SoundPlayer {
    output: Option<(MixerDeviceSink, Player)>,
}

impl SoundPlayer {
    /// 新しい効果音プレイヤーを作成
    pub fn new() -> Self {
        // デフォルトのサウンド出力を開く
        let output = match rodio::DeviceSinkBuilder::open_default_sink() {
            Ok(handle) => {
                let player = Player::connect_new(handle.mixer());
                Some((handle, player))
            }
            Err(e) => {
                log::warn!("Failed to open default sink: {}", e);
                None
            }
        };

        Self { output }
    }

    /// 効果音を再生
    /// * `bytes` - 効果音のデータ
    fn play_sound(&self, bytes: &'static [u8]) {
        let Some((_, player)) = &self.output else { return };
        player.stop();
        if let Ok(source) = Decoder::try_from(Cursor::new(bytes)) {
            player.append(source);
        }
    }

    /// 効果音の音量を設定
    /// * `volume` - 効果音の音量
    pub fn set_volume(&self, volume: u8) {
        let Some((_, player)) = &self.output else { return };
        player.set_volume(volume as f32 / 10.0);
    }

    /// 完了時の効果音を再生
    pub fn play_completed(&self) {
        self.play_sound(COMPLETED.bytes);
    }

    /// エラー時の効果音を再生
    pub fn play_alert(&self) {
        self.play_sound(ALERT.bytes);
    }
}
