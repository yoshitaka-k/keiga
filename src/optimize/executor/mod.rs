mod jpeg;
mod png;

pub(crate) use jpeg::Jpeg;
pub(crate) use png::Png;

use std::path::Path;
use image::{self, ImageDecoder, DynamicImage};
use crate::{error, file};
use crate::optimize;
use crate::optimize::guard;

/// 画像を読み込んだ結果
/// * `image` - 読み込んだ画像
/// * `exif` - EXIF メタデータ（ Orientation をリセットしたもの）
/// * `icc` - ICC プロファイル
pub(crate) struct OpenImageResult {
    pub image: DynamicImage,
    pub exif: Option<Vec<u8>>,
    pub icc: Option<Vec<u8>>,
}

/// 最適化を行うトレイト
pub trait Optimizer {
    type Options;

    /// ファイルをエンコードする
    /// * `path` - エンコードするファイルのパス
    /// * `options` - エンコードオプション
    /// * `return` - エンコードされたファイルのサイズとデータ
    fn encode(
        path: &Path,
        options: Self::Options,
    ) -> error::Result<(usize, Vec<u8>)>;

    /// Lossy 最適化
    /// * `path` - 最適化するファイルのパス
    /// * `options` - 最適化オプション
    /// * `return` - エンコードされたファイルのサイズとデータ
    fn lossy(
        path: &Path,
        options: Self::Options
    ) -> error::Result<(usize, Vec<u8>)>;

    /// Lossless 最適化
    /// * `path` - 最適化するファイルのパス
    /// * `options` - 最適化オプション
    /// * `return` - エンコードされたファイルのサイズとデータ
    fn lossless(
        path: &Path,
        options: Self::Options
    ) -> error::Result<(usize, Vec<u8>)>;

    /// 画像を読み込む
    /// * `path` - 読み込む画像のパス
    /// * `return` - 読み込んだ画像
    fn open_image(path: &Path) -> error::Result<OpenImageResult> {
        // 画像を読み込む
        let reader = image::ImageReader::open(path)
            .and_then(|r| r.with_guessed_format())
            .map_err(|e| error::KeigaError::optimized_error(e.to_string(), path.to_path_buf()))?;

        // リーダーをデコーダーに変換
        let mut decoder = reader.into_decoder().map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?;

        // 情報を取得
        // 向きのメタデータを取得
        let orientation = decoder.orientation().map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?;

        // EXIF メタデータを取得
        let mut exif = decoder.exif_metadata().map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?;

        // EXIF の Orientation をリセット
        if let Some(exif) = exif.as_mut() {
            let _ = image::metadata::Orientation::remove_from_exif_chunk(exif);
        }

        // ICC プロファイルを取得
        // image は CMYK を RGB に変換してデコードするため、CMYK 用の ICC は付けない
        let icc = decoder.icc_profile().map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?.filter(|icc| icc.get(16..20) != Some(b"CMYK"));

        // 画像をデコードからピクセルデータへ変換
        let mut image = DynamicImage::from_decoder(decoder).map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), path.to_path_buf())
        })?;

        // 画像の向きを適用
        image.apply_orientation(orientation);

        Ok(OpenImageResult {
            image,
            exif,
            icc,
        })
    }

    /// 最適化を行う
    /// * `path` - 最適化するファイルのパス
    /// * `output_path` - 出力ファイルのパス
    /// * `options` - 最適化オプション
    /// * `token` - 最適化トークン
    /// * `return` - 最適化の結果
    fn execute(
        path: &Path,
        output_path: &Path,
        options: Self::Options,
        token: optimize::OptimToken
    ) -> error::Result<file::Status> {
        // 最適化中止された場合は処理を中断
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        // ファイルを種類に応じてエンコードする
        let (original_size, byte_data) = Self::encode(path, options)?;

        // 最適化中止された場合は処理を中断
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        // 最適化後のサイズが元のサイズより大きい場合は最適化しない
        let new_size = byte_data.len();
        if original_size <= new_size {
            // 出力ファイルが存在しない場合は作成、エラーが発生した場合はエラーを返す
            match output_path.try_exists() {
                Ok(true) => (),
                Ok(false) => {
                    if path != output_path {
                        // 出力ファイルのパスを作成
                        create_output_path(output_path)?;

                        // ファイルをコピー
                        std::fs::copy(path, output_path).map_err(|e| {
                            error::KeigaError::optimized_error(e.to_string(), output_path.to_path_buf())
                        })?;
                    }
                }
                Err(e) => {
                    return Err(error::KeigaError::optimized_error(e.to_string(), output_path.to_path_buf()));
                }
            }

            return Ok(file::Status::Unchanged);
        }

        // 出力ファイルのパスを作成
        create_output_path(output_path)?;

        // 一時ファイルを作成して最適化後のデータを保存
        let temp_path = output_path.with_added_extension(optimize::TEMP_EXTENSION);

        // 一時ファイルを削除するためのガードを作成
        let mut guard = guard::OptimizeGuard::new(temp_path.clone());

        // 一時ファイルを作成して最適化後のデータを保存
        std::fs::write(&temp_path, &byte_data).map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), temp_path.to_path_buf())
        })?;

        // 最適化中止された場合は処理を中断
        if token.is_canceled()? { return Ok(file::Status::Canceled); }

        // 一時ファイルを元のファイルに上書き
        replace_file(&temp_path, output_path)?;

        // 削除しない
        guard.keep();

        Ok(file::Status::Optimized)
    }
}

/// 出力ファイルのパスを作成
/// * `output_path` - 出力パス
/// * `return` - 出力ファイルのパスが作成できたかどうか
fn create_output_path(output_path: &Path) -> error::Result<()> {
    // 出力ファイルのパスの親ディレクトリを取得
    let Some(parent) = output_path.parent() else {
        return Err(error::KeigaError::optimized_error("Output path parent not found".to_string(), output_path.to_path_buf()));
    };

    // 出力ファイルのパスの親ディレクトリを作成
    std::fs::create_dir_all(parent).map_err(|e| {
        error::KeigaError::optimized_error(e.to_string(), output_path.to_path_buf())
    })
}

/// 一時ファイルを元のファイルに上書き
/// * `from` - 一時ファイルのパス
/// * `to` - 元のファイルのパス
/// * `return` - 一時ファイルを元のファイルに上書きしたかどうか
fn replace_file(from: &Path, to: &Path) -> error::Result<()> {
    // Windows 以外の環境ではファイルを直接上書き
    #[cfg(not(target_os = "windows"))]
    {
        std::fs::rename(from, to).map_err(|e| {
            error::KeigaError::optimized_error(e.to_string(), to.to_path_buf())
        })?;
    }

    // Windows 環境では MoveFileExW を使用してファイルを上書き
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::ffi::OsStrExt;
        use winapi::um::winbase::{MoveFileExW, MOVEFILE_WRITE_THROUGH, MOVEFILE_REPLACE_EXISTING};

        // Windows の API に渡すためにパスを UTF-16 に変換
        let from_win: Vec<u16> = from.as_os_str().encode_wide().chain(Some(0)).collect();
        let to_win: Vec<u16> = to.as_os_str().encode_wide().chain(Some(0)).collect();

        // MoveFileExW を呼び出してファイルを上書き
        let result = unsafe {
            MoveFileExW(from_win.as_ptr(), to_win.as_ptr(), MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH)
        };

        // エラーが発生した場合はエラーを返す
        if result == 0 {
            return Err(error::KeigaError::optimized_error(std::io::Error::last_os_error().to_string(), to.to_path_buf()));
        }
    }
    Ok(())
}
