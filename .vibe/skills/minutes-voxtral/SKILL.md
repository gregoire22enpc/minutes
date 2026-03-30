---
name: minutes-voxtral
description: >
  Set up Voxtral (Mistral's speech-to-text model) as the transcription engine
  for Minutes. Use when the user says "use Voxtral", "switch to Voxtral",
  "set up Voxtral", "Mistral transcription", or wants lower word error rates
  than Whisper on supported languages.
user-invocable: true
allowed-tools:
  - bash
  - read_file
  - write_file
---

# /minutes voxtral

Set up Voxtral as the transcription engine for Minutes, replacing whisper.cpp.

## Why Voxtral?

Voxtral Realtime is Mistral's open-weight speech-to-text model (Apache 2.0).
It achieves lower word error rates than Whisper large-v3 on supported languages,
with ~2.5x real-time speed on Apple Silicon.

Supports 13 languages: English, Spanish, French, Portuguese, Hindi, German,
Dutch, Italian, Arabic, Russian, Chinese, Japanese, Korean.

## Setup steps

### 1. Build voxtral.c

[voxtral.c](https://github.com/antirez/voxtral.c) is a pure C inference engine
for Voxtral (like whisper.cpp is for Whisper).

```bash
git clone https://github.com/antirez/voxtral.c
cd voxtral.c

# Apple Silicon (Metal GPU — fastest)
make mps

# macOS Intel or Linux (requires OpenBLAS)
# Linux: sudo apt install libopenblas-dev
make blas
```

### 2. Download the model (~8.9GB)

```bash
./download_model.sh
```

### 3. Verify it works

```bash
# Test with any audio file
./voxtral -d voxtral-model -i /path/to/test.wav
```

Tokens should stream to stdout. On Apple M3 Max, expect ~284ms encoder latency
for 3.6s of audio.

### 4. Configure Minutes

Add to `~/.config/minutes/config.toml`:

```toml
[transcription]
engine = "voxtral"
voxtral_binary = "/path/to/voxtral.c/voxtral"
voxtral_model = "/path/to/voxtral.c/voxtral-model"
```

### 5. Test a recording

```bash
minutes record --title "Voxtral test"
# speak for a few seconds
minutes stop
```

Check the output transcript quality in `~/meetings/`.

## Switching back to Whisper

```toml
[transcription]
engine = "whisper"   # or remove the [transcription] section entirely
```

## Voxtral vs Whisper vs Parakeet

| Engine | Model size | Languages | Strengths |
|--------|-----------|-----------|-----------|
| Whisper (default) | 75MB–3.1GB | 99 languages | Broadest language support, mature ecosystem |
| Voxtral | ~8.9GB | 13 languages | Lowest WER on supported languages, streaming, Mistral ecosystem |
| Parakeet | 220MB–1.2GB | 25 EU languages | Low WER for English, NVIDIA-optimized |

## Gotchas

- **Model is ~8.9GB** — much larger than Whisper tiny/small. Make sure you have disk space.
- **13 languages only** — if you need languages outside the supported set, stick with Whisper.
- **voxtral.c is newer** — whisper.cpp has a larger community and more edge cases handled.
- **macOS MPS recommended** — CPU-only inference is significantly slower.
- **ffmpeg recommended** — for non-WAV input, voxtral.c pipes through ffmpeg for conversion.
