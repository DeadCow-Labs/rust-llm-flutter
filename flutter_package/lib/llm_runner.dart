import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'bindings.dart';

class LlmRunner {
  static late DynamicLibrary _lib;

  /// Initializes the Rust library
  static void init() {
    _lib = Platform.isAndroid
        ? DynamicLibrary.open('libllm_runner.so')
        : DynamicLibrary.open('libllm_runner.dylib');
  }

  /// Downloads a model (LLaMA 3, DeepSeek, etc.)
  static Future<void> downloadModel(String modelName) async {
    final modelNamePtr = modelName.toNativeUtf8();
    bindings.download_model(modelNamePtr.cast());
    calloc.free(modelNamePtr);
  }

  /// Loads a model into memory
  static Future<void> loadModel(String modelName) async {
    final modelNamePtr = modelName.toNativeUtf8();
    bindings.load_model(modelNamePtr.cast());
    calloc.free(modelNamePtr);
  }

  /// Runs inference on the loaded model
  static Future<String> runInference(String prompt) async {
    final promptPtr = prompt.toNativeUtf8();
    Pointer<Utf8> resultPtr = bindings.run_inference(promptPtr.cast());
    String result = resultPtr.toDartString();
    calloc.free(promptPtr);
    return result;
  }
}
