# Tested Models

This document lists models that have been tested with LLM Runner.

## Officially Supported

### TinyLlama (4GB+ RAM)
- Name: `TinyLlama/TinyLlama-1.1B-Chat-v0.6`
- Size: ~1GB
- Good for: Basic chat, simple tasks
- Performance: ~20 tokens/second

### Phi-2 (4GB+ RAM)
- Name: `microsoft/phi-2`
- Size: ~1.5GB
- Good for: Coding, reasoning
- Performance: ~18 tokens/second

### Llama 3.2 3B (6GB+ RAM)
- Name: `meta-llama/Llama-3.2-3b-instruct`
- Size: ~2GB
- Good for: General tasks, instruction following
- Performance: ~15 tokens/second

## Community Tested

Users have reported success with:

### DeepSeek Models
- DeepSeek R1 Distill Llama 8B
- DeepSeek R1 Distill Qwen 7B
- Requirements: 8GB+ RAM

### Gemma Models
- Gemma-2 2B IT
- Gemma-2 9B IT
- Requirements: 4GB/16GB RAM

### Qwen Models
- Qwen 2.5 1.5B
- Qwen 2.5 7B
- Requirements: 4GB/8GB RAM

## Adding New Models

To add a new model:

1. Create a custom configuration:
```dart
final myModel = Models.custom(
  name: 'model-org/model-name',
  minRamMb: requiredRam,
  description: 'Model description',
);
```

2. Test compatibility:
- Memory usage
- Inference speed
- Output quality

3. Share your results:
- Open an issue
- Submit a PR to update this list 