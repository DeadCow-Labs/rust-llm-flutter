import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'package:path/path.dart' as path;
import 'src/bindings.dart';

class LlmRunner {
  static final tinyllama = ModelConfig('TinyLlama/TinyLlama-1.1B-Chat-v0.6');
  static final phi15 = ModelConfig('microsoft/phi-1_5');

  static ModelConfig _currentModel = tinyllama;
  static bool _isModelLoaded = false;

  static Future<String> generateText({
    required ModelConfig model,
    required String prompt,
  }) async {
    print('\n=== Generate Text ===');
    print('Requested model: ${model.name}');
    print('Current model: ${_currentModel.name}');
    print('Is model loaded: $_isModelLoaded');
    
    // Check if we need to switch models
    if (_currentModel != model || !_isModelLoaded) {
      print('Need to switch models');
      print('Switching from ${_currentModel.name} to ${model.name}');
      await _switchModel(model);
      _currentModel = model;
      _isModelLoaded = true;
      print('Model switch complete');
    } else {
      print('Using already loaded model: ${model.name}');
    }
    
    try {
      print('Running inference...');
      final input = prompt.toNativeUtf8();
      final resultPtr = _runInference(input) as Pointer<Utf8>;
      final response = resultPtr.toDartString();
      
      calloc.free(input);
      _freeString(resultPtr);
      
      return response;
    } catch (e) {
      print('Error during inference: $e');
      throw Exception('Failed to generate response: $e');
    }
  }

  // Private methods
  static bool _isInitialized = false;
  static late final DynamicLibrary _lib;
  static late final LoadModelC _loadModelFn;
  static late final RunInferenceC _runInference;
  static late final FreeStringC _freeString;
  static late final DownloadModelC _downloadModelFn;

  static Future<void> _switchModel(ModelConfig model) async {
    print('\n=== Switching Model ===');
    print('Target model: ${model.name}');
    
    await _initializeIfNeeded();
    
    try {
      // First, download the model if needed
      print('Downloading model if needed: ${model.name}');
      final downloadPtr = _downloadModelFn(model.name.toNativeUtf8());
      final downloadResult = downloadPtr.toDartString();
      print('Download result: $downloadResult');
      _freeString(downloadPtr);

      // Then load the model
      print('Loading model: ${model.name}');
      final modelName = model.name.toNativeUtf8();
      final loadResultPtr = _loadModelFn(modelName);
      final loadResult = loadResultPtr.toDartString();
      print('Load result: $loadResult');
      
      calloc.free(modelName);
      _freeString(loadResultPtr);
      
      print('Model switch complete');
    } catch (e) {
      print('Error during model switch: $e');
      rethrow;
    }
  }

  static Future<void> _initializeIfNeeded() async {
    if (_isInitialized) return;

    _lib = DynamicLibrary.open(_getLibraryPath());
    
    _loadModelFn = _lib.lookupFunction<LoadModelC, LoadModelC>('load_model_c');
    _runInference = _lib.lookupFunction<RunInferenceC, RunInferenceC>('run_inference_c');
    _freeString = _lib.lookupFunction<FreeStringRust, FreeStringC>('free_string');
    _downloadModelFn = _lib.lookupFunction<DownloadModelC, DownloadModelC>('download_model_ffi');

    _isInitialized = true;
  }

  static String _getLibraryPath() {
    // Get the current directory
    final currentDir = Directory.current;
    print('Current directory: ${currentDir.path}');
    
    // Try to find the library in several possible locations
    final possiblePaths = [
      // If running from the main repo
      path.join(
        currentDir.parent.parent.path,
        'llm_runner',
        'rust',
        'target',
        'release',
      ),
      // If running from examples repo
      path.join(
        currentDir.parent.parent.parent.path,
        'llm_runner',
        'rust',
        'target',
        'release',
      ),
    ];

    final libraryName = Platform.isMacOS 
      ? 'libllm_runner.dylib'
      : Platform.isWindows 
        ? 'llm_runner.dll' 
        : 'libllm_runner.so';

    for (final basePath in possiblePaths) {
      final libraryPath = path.join(basePath, libraryName);
      print('Checking library at: $libraryPath');
      
      if (File(libraryPath).existsSync()) {
        print('Found library at: $libraryPath');
        return libraryPath;
      }
    }

    throw Exception('''
Library not found. 
Make sure:
1. You've run 'cargo build --release' in the llm_runner/rust directory
2. The directory structure is correct
3. The library file exists at one of these paths:
   ${possiblePaths.map((p) => path.join(p, libraryName)).join('\n   ')}

Current directory: ${currentDir.path}
''');
  }

  /// Downloads a model (TinyLlama, Phi-1.5, etc.)
  static Future<void> downloadModel(String modelName) async {
    final modelNamePtr = modelName.toNativeUtf8();
    final resultPtr = _loadModelFn(modelNamePtr);
    final result = resultPtr.toDartString();
    print('Download result: $result');
    
    calloc.free(modelNamePtr);
    _freeString(resultPtr);
  }

  /// Loads a model into memory
  static Future<void> loadModel(String modelName) async {
    final modelNamePtr = modelName.toNativeUtf8();
    final resultPtr = _loadModelFn(modelNamePtr);
    final result = resultPtr.toDartString();
    print('Load result: $result');
    
    calloc.free(modelNamePtr);
    _freeString(resultPtr);
  }

  /// Runs inference on the loaded model
  static Future<String> runInference(String prompt) async {
    final promptPtr = prompt.toNativeUtf8();
    final resultPtr = _runInference(promptPtr);
    final response = resultPtr.toDartString();
    
    calloc.free(promptPtr);
    _freeString(resultPtr);
    
    return response;
  }
}
