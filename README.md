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

// Generate text with TinyLlama
final response = await LlmRunner.generateText(
  model: LlmRunner.tinyllama,
  prompt: "Tell me a joke",
);
print(response);

// Switch to Phi-1.5 for math
final mathResponse = await LlmRunner.generateText(
  model: LlmRunner.phi15,
  prompt: "What is 2+2?",
);
print(mathResponse);
```

## 🔧 Available Models

- **TinyLlama**: Great for general text generation
  ```dart
  LlmRunner.tinyllama
  ```

- **Phi-1.5**: Excellent for math and reasoning
  ```dart
  LlmRunner.phi15
  ```

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
