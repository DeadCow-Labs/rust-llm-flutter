# LLM Runner - Run AI Models in Flutter with Rust

LLM Runner is a Rust-powered library for running local AI models (like TinyLlama, Phi-1.5) in Flutter apps. It handles model downloading, loading, and inference with a simple API.

## 🚀 Features
- **Multiple Models**: Support for TinyLlama, Phi-1.5, and more
- **Automatic Downloads**: Models are downloaded automatically when needed
- **Local Execution**: All processing happens on device
- **Memory Efficient**: Models are loaded/unloaded as needed
- **Simple API**: Just a few lines to get started

## 📦 Installation

Add to your `pubspec.yaml`:
```yaml
dependencies:
  llm_runner:
    git: https://github.com/yourusername/rust_llm_runner.git
```

## 🎯 Quick Start

```dart
import 'package:llm_runner/llm_runner.dart';

// Use a pre-configured model
final response = await LlmRunner.generateText(
  model: Models.tinyllama,  // Small, fast model
  prompt: "Tell me a joke",
);

// Switch to a more powerful model
final mathResponse = await LlmRunner.generateText(
  model: Models.mistral7b,  // Better at complex tasks
  prompt: "Explain quantum computing",
);

// Use your own custom model
final customModel = Models.custom(
  name: 'deepseek-ai/deepseek-math-7b-instruct',
  minRamMb: 8192,
  description: 'Specialized for mathematics',
);

final mathResult = await LlmRunner.generateText(
  model: customModel,
  prompt: "Solve: ∫x²dx",
);
```

## 📱 Available Models

### Small Models (4GB+ RAM)
- `Models.tinyllama` - Fast, lightweight
- `Models.phi2` - Good at coding
- `Models.gemma2b` - Google's efficient model

### Medium Models (6GB+ RAM)
- `Models.llama32_3b` - Latest Llama 3.2
- `Models.mistral7b` - Powerful open-source

### Large Models (8GB+ RAM)
- `Models.qwen7b` - High-quality multilingual

### Custom Models
Use any compatible model:
```dart
final myModel = Models.custom(
  name: 'organization/model-name',
  minRamMb: 6144,
  description: 'My custom model',
  metadata: {
    'type': 'instruct',
    'language': 'multilingual',
  },
);
```

## 🔍 Model Compatibility

Models should be:
1. GGUF format compatible
2. Within device memory constraints
3. Properly structured (tokenizer, weights, etc.)

See [MODELS.md](MODELS.md) for a full list of tested models.

## 🚦 Advanced Usage

### Model Switching
Models are automatically downloaded and loaded as needed:

```dart
// Use TinyLlama
var response = await LlmRunner.generateText(
  model: LlmRunner.tinyllama,
  prompt: "Tell me a story",
);

// Switch to Phi-1.5
response = await LlmRunner.generateText(
  model: LlmRunner.phi15,
  prompt: "Solve: x^2 = 16",
);
```

### Error Handling
```dart
try {
  final response = await LlmRunner.generateText(
    model: LlmRunner.tinyllama,
    prompt: "Hello!",
  );
  print(response);
} catch (e) {
  print('Error: $e');
}
```

## 🔍 How It Works

1. **Model Management**: The library automatically handles:
   - Model downloading
   - Loading into memory
   - Efficient switching between models
   - Memory cleanup

2. **Performance**: 
   - ~50ms per token generation
   - ~20 tokens per second
   - Automatic memory management

## 📝 Requirements

- Flutter 3.0 or higher
- iOS 11+ or Android 21+
- ~500MB free storage per model
- ~1GB RAM for model execution

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details
