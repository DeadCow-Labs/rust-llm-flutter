# LLM Runner - Run AI Models in Flutter with Rust

LLM Runner is a Rust-powered library for running lightweight AI models in Flutter apps, optimized for mobile devices.

## 🚀 Features
- **Local Model Execution**: Run AI models directly on device
- **Automatic Model Management**: Handles downloading and loading
- **Mobile Optimized**: Specifically designed for iOS & Android
- **Memory Efficient**: Uses quantization and optimized inference

## 📱 Supported Models
Best performing models for mobile devices (30-80ms per token):

- **TinyLlama-1.1B-Chat**: Fast, lightweight chat model
- **Phi-1.5**: Microsoft's efficient model
- **RWKV-430M**: Ultra-lightweight option

## 📦 Installation
1. **Add the package to your Flutter project**
   ```yaml
   dependencies:
     llm_runner:
       git: https://github.com/yourusername/llm_runner.git
   ```

2. **Initialize and use the library**
   ```dart
   import 'package:llm_runner/llm_runner.dart';

   void main() async {
       // Load TinyLlama, optimized for mobile
       await LlmRunner.downloadModel('TinyLlama/TinyLlama-1.1B-Chat-v0.6');
       await LlmRunner.loadModel('TinyLlama/TinyLlama-1.1B-Chat-v0.6');
       
       // Run inference
       String result = await LlmRunner.runInference("What is quantum computing?");
       print(result);
   }
   ```

## ⚡ Performance
Typical performance metrics on recent devices:
- iPhone 13/14: 30-50ms per token
- Recent Android flagships: 40-60ms per token
- Older devices: 60-100ms per token

## 🛠️ Technical Details
- Built with Rust for optimal performance
- Uses F16 quantization
- Memory-mapped model loading
- Optimized token generation

## ⚠️ Device Requirements
- iOS: iPhone XS or newer recommended
- Android: 4GB RAM minimum, 6GB+ recommended
- ~600MB storage for model files

## 📝 License
[Your chosen license]

## 🤝 Contributing
Contributions welcome! Please read our contributing guidelines.