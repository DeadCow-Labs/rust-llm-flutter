# LLM Runner - Run AI Models in Flutter with Rust

LLM Runner is a Rust-powered library for running local AI models (like LLaMA 3, DeepSeek) in Flutter apps.

## 🚀 Features
- **Local Model Execution**: No need for external APIs.
- **Automatic Model Downloading**: Just specify the model, and it gets downloaded.
- **Optimized for Mobile**: Works on iOS & Android with FFI.

## 📦 Installation
1. **Add the package to your Flutter project**
   ```yaml
   dependencies:
     llm_runner:
       git: https://github.com/yourusername/llm_runner.git
   ```

2. **Initialize the library**
   ```dart
    import 'package:llm_runner/llm_runner.dart';

    void main() async {
        await LlmRunner.downloadModel('llama3');
        await LlmRunner.loadModel('llama3');
        String result = await LlmRunner.runInference("Hello, AI!");
        print(result);
    }

   ```

3. **Run the inference**
   ```dart
   String result = await LlmRunner.runInference("Hello, AI!");
   print(result);
   ```