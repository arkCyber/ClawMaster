# ClawMaster Voice

语音功能模块，提供文本转语音（TTS）和语音转文本（STT）能力。

## 功能特性

### 🎤 语音转文本（STT）

支持 9 个提供商：

- **OpenAI Whisper** - 高质量云端转录
- **Groq** - 超快速 Whisper 推理
- **Deepgram** - 实时转录专家
- **Google Cloud** - 企业级语音识别
- **Mistral AI** - Voxtral 模型
- **ElevenLabs Scribe** - 高精度转录
- **Voxtral (本地)** - 通过 vLLM 本地运行
- **whisper.cpp** - 轻量级本地转录
- **sherpa-onnx** - 离线语音识别

### 🔊 文本转语音（TTS）

支持 5 个提供商：

- **ElevenLabs** - 最自然的 AI 语音
- **OpenAI TTS** - 高质量神经语音
- **Google Cloud TTS** - 多语言支持
- **Piper** - 快速本地合成
- **Coqui TTS** - 开源语音克隆

### ✨ 新增功能

#### 流式 TTS 支持
- 低延迟音频传输
- 分块音频流
- 性能指标跟踪
- 自适应缓冲

#### 语音活动检测（VAD）
- 基于能量的 VAD
- 语音段检测
- 静音修剪
- 可配置阈值

#### 音频处理工具
- 音量归一化
- 采样率转换
- 立体声/单声道转换
- 淡入淡出效果
- 格式检测
- 音频质量预设

## 快速开始

### 基本使用

```rust
use clawmaster_voice::{
    TtsProvider, SttProvider,
    OpenAiTts, WhisperStt,
    SynthesizeRequest, TranscribeRequest,
    AudioFormat,
};

// TTS 示例
let tts = OpenAiTts::new(config);
let request = SynthesizeRequest {
    text: "你好，世界！".to_string(),
    voice_id: Some("alloy".to_string()),
    output_format: AudioFormat::Mp3,
    ..Default::default()
};
let audio = tts.synthesize(request).await?;

// STT 示例
let stt = WhisperStt::new(config);
let request = TranscribeRequest {
    audio: audio_bytes,
    format: AudioFormat::Mp3,
    language: Some("zh".to_string()),
    prompt: None,
};
let transcript = stt.transcribe(request).await?;
println!("转录结果: {}", transcript.text);
```

### 语音活动检测

```rust
use clawmaster_voice::{EnergyVad, VadConfig};

let config = VadConfig {
    min_speech_duration: 0.3,
    max_silence_duration: 0.8,
    energy_threshold: 0.02,
    sample_rate: 16000,
};

let vad = EnergyVad::new(config);
let result = vad.detect(&audio_samples, 16000)?;

if result.has_speech {
    println!("检测到语音，置信度: {:.2}", result.confidence);
}
```

### 音频处理

```rust
use clawmaster_voice::{
    normalize_volume, resample, stereo_to_mono,
    apply_fade_in, apply_fade_out,
};

// 音量归一化
let mut samples = load_audio_samples();
normalize_volume(&mut samples, 0.8);

// 采样率转换
let resampled = resample(&samples, 8000, 16000)?;

// 立体声转单声道
let mono = stereo_to_mono(&stereo_samples)?;

// 添加淡入淡出效果
apply_fade_in(&mut samples, 1600);  // 100ms at 16kHz
apply_fade_out(&mut samples, 1600);
```

### 流式 TTS

```rust
use clawmaster_voice::streaming::{StreamingTtsProvider, StreamingMetrics};
use futures::StreamExt;

let provider = get_streaming_provider();
let mut stream = provider.synthesize_stream(request).await?;
let mut metrics = StreamingMetrics::new();

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    metrics.record_chunk(&chunk);
    
    // 播放或处理音频块
    play_audio_chunk(&chunk.data);
    
    if chunk.is_final {
        break;
    }
}

println!("首字节延迟: {:?}ms", metrics.time_to_first_chunk_ms);
```

## 配置示例

### TTS 配置

```toml
[voice.tts]
enabled = true
provider = "elevenlabs"
auto = "off"
max_text_length = 2000

[voice.tts.elevenlabs]
api_key = "sk-..."
voice_id = "21m00Tcm4TlvDq8ikWAM"
model = "eleven_flash_v2_5"
stability = 0.5
similarity_boost = 0.75

[voice.tts.openai]
voice = "alloy"
model = "tts-1"
speed = 1.0

[voice.tts.piper]
model_path = "~/.clawmaster/models/en_US-lessac-medium.onnx"
```

### STT 配置

```toml
[voice.stt]
enabled = true
provider = "whisper"

[voice.stt.whisper]
model = "whisper-1"
language = "zh"

[voice.stt.groq]
api_key = "gsk_..."
model = "whisper-large-v3-turbo"

[voice.stt.whisper_cli]
model_path = "~/.clawmaster/models/ggml-base.en.bin"
language = "en"
```

## 音频质量预设

```rust
use clawmaster_voice::AudioQuality;

// 低质量 (8kHz, 单声道, 64kbps)
let low = AudioQuality::Low;

// 中等质量 (16kHz, 单声道, 128kbps)
let medium = AudioQuality::Medium;

// 高质量 (24kHz, 单声道, 192kbps)
let high = AudioQuality::High;

// 录音室质量 (48kHz, 立体声, 320kbps)
let studio = AudioQuality::Studio;
```

## 测试

运行所有测试：

```bash
cargo test -p clawmaster-voice
```

运行集成测试：

```bash
cargo test -p clawmaster-voice --test integration_tests
```

运行示例：

```bash
cargo run --example voice_demo -p clawmaster-voice
```

## 架构

```
clawmaster-voice/
├── src/
│   ├── lib.rs              # 模块入口
│   ├── config.rs           # 配置类型
│   ├── tts/                # TTS 实现
│   │   ├── mod.rs          # TTS trait
│   │   ├── streaming.rs    # 流式 TTS
│   │   ├── elevenlabs.rs   # ElevenLabs
│   │   ├── openai.rs       # OpenAI
│   │   ├── google.rs       # Google
│   │   ├── piper.rs        # Piper (本地)
│   │   └── coqui.rs        # Coqui (本地)
│   ├── stt/                # STT 实现
│   │   ├── mod.rs          # STT trait
│   │   ├── whisper.rs      # OpenAI Whisper
│   │   ├── groq.rs         # Groq
│   │   ├── deepgram.rs     # Deepgram
│   │   ├── google.rs       # Google
│   │   ├── mistral.rs      # Mistral
│   │   ├── elevenlabs.rs   # ElevenLabs
│   │   ├── voxtral_local.rs # Voxtral (本地)
│   │   ├── whisper_cli.rs  # whisper.cpp
│   │   └── sherpa_onnx.rs  # sherpa-onnx
│   ├── vad.rs              # 语音活动检测
│   └── audio_utils.rs      # 音频处理工具
├── examples/
│   └── voice_demo.rs       # 使用示例
└── tests/
    └── integration_tests.rs # 集成测试
```

## 性能优化

### TTS 优化
- 使用流式 TTS 降低首字节延迟
- 选择合适的音频格式（Opus 适合实时，MP3 适合存储）
- 启用块优化以减少网络往返

### STT 优化
- 使用 VAD 预处理音频，减少 API 调用
- 修剪静音以减少传输数据量
- 选择合适的模型大小（速度 vs 准确度）

### 音频处理优化
- 使用适当的采样率（16kHz 适合语音）
- 批量处理音频以提高效率
- 考虑使用硬件加速（GPU）进行本地推理

## 安全性

- API 密钥使用 `secrecy::Secret` 保护
- 调试输出自动隐藏敏感信息
- 支持环境变量和配置文件两种方式

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License - 详见 [LICENSE.md](../../LICENSE.md)
