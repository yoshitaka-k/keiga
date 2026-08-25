mod generated {
    use crate::rendar::assets::sounds::SoundSource;
    include!(concat!(env!("OUT_DIR"), "/sounds_generated.rs"));
}

pub use generated::*;

/// 埋め込み効果音データ
pub struct SoundSource {
    pub bytes: &'static [u8],
}

impl SoundSource {
    /// 新しい効果音データを作成する
    pub const fn new(bytes: &'static [u8]) -> Self {
        Self { bytes }
    }
}

use std::io::Cursor;
use rodio::{Decoder, MixerDeviceSink, Player};
use crate::rendar::assets::sounds;

/// 効果音プレイヤー
pub struct SoundPlayer {
    _stream: MixerDeviceSink,
    player: Player,
}

impl SoundPlayer {
    /// 新しい効果音プレイヤーを作成
    pub fn new() -> Self {
        let handle = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
        let player = rodio::Player::connect_new(&handle.mixer());

        Self { _stream: handle, player }
    }

    /// 効果音の音量を設定
    /// * `volume` - 効果音の音量
    pub fn set_volume(&self, volume: u8) {
        let volume = volume as f32 / 10.0;
        self.player.set_volume(volume);
    }

    /// 完了時の効果音を再生
    pub fn play_completed(&self) {
        self.play_sound(&sounds::COMPLETED.bytes);
    }

    /// エラー時の効果音を再生
    pub fn play_alert(&self) {
        self.play_sound(&sounds::ALERT.bytes);
    }

    /// 効果音を再生
    /// * `bytes` - 効果音のデータ
    fn play_sound(&self, bytes: &'static [u8]) {
        self.player.stop();
        if let Ok(source) = Decoder::try_from(Cursor::new(bytes)) {
            self.player.append(source);
        }
    }
}
