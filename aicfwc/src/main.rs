use anyhow::{Context, Result, bail};
use clap::{ArgAction, Parser};
use std::{fs, io::Write, path::PathBuf};

/// AIC firmware converter.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// The input binary file.
    input: PathBuf,

    /// Output as PBP (Pre-Boot Program) format.
    ///
    /// PBP format:
    /// [0..=3]  = b"PBP ",
    /// [4..=7]  = checksum (u32 LE),
    /// [8..end] = original binary 4-byte aligned with zero paddings.
    #[arg(long = "pbp", action = ArgAction::SetTrue)]
    pbp: bool,

    /// Output file path.
    #[arg(short = 'o', long = "output")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 读取输入bin
    let bin_data =
        fs::read(&cli.input).with_context(|| format!("无法读取输入文件 {:?}", cli.input))?;

    // 目前我们只实现 -pbp 模式；如果没加 -pbp，就报错
    if !cli.pbp {
        bail!("当前仅支持 -pbp 预处理，请加 -pbp 开关");
    }

    let pbp_bytes = build_pbp(&bin_data)?;

    // 写出文件
    let mut f = fs::File::create(&cli.output)
        .with_context(|| format!("无法创建输出文件 {:?}", cli.output))?;
    f.write_all(&pbp_bytes)
        .with_context(|| format!("无法写入输出文件 {:?}", cli.output))?;

    Ok(())
}

/// 构造PBP格式:
/// 0..=3:  "PBP "
/// 4..=7:  checksum (稍后写回)
/// 8..end: bin内容+补零到4字节对齐
///
/// checksum计算:
///   把最终整个PBP文件按<u32小端>分成若干word相加(模2^32)，
///   要求最后总和 == 0x0fffffff。
///   我们先把checksum位置当0算出partial_sum，
///   再倒推出checksum。
fn build_pbp(bin: &[u8]) -> Result<Vec<u8>> {
    // 1. 对齐bin到4字节
    let mut aligned = bin.to_vec();
    while aligned.len() % 4 != 0 {
        aligned.push(0);
    }

    // 2. 先拼一个占位的PBP: magic + checksum占位 + 数据
    let total_len = aligned.len();
    let mut out = Vec::with_capacity(total_len);

    // 数据
    out.extend_from_slice(&aligned);

    // 3. 计算当前(占位checksum=0)的32位和
    let mut sum: u64 = 0;
    for chunk in out.chunks_exact(4) {
        let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        sum = (sum + word as u64) & 0xffff_ffff;
    }

    // 4. 倒推出真正的checksum
    let target: u32 = 0xffff_ffff;
    let checksum = target.wrapping_sub(sum as u32);

    // 5. 写回checksum (小端)
    out[4..8].copy_from_slice(&checksum.to_le_bytes());

    // 6. 可选: 断言一下正确性（debug用）
    debug_assert!(verify_checksum(&out, target));

    Ok(out)
}

/// 校验最终文件的32位和是不是 target
fn verify_checksum(buf: &[u8], target: u32) -> bool {
    if buf.len() % 4 != 0 {
        return false;
    }
    let mut sum: u64 = 0;
    for chunk in buf.chunks_exact(4) {
        let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        sum = (sum + word as u64) & 0xffff_ffff;
    }
    (sum as u32) == target
}
