import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'package:path/path.dart' as path;
import 'src/bindings.dart';
import 'src/models_config.dart';  // Import everything from models_config

class LlmRunner {
  // Remove the static model definitions since they're now in Models class
  static ModelConfig _currentModel = Models.tinyllama;  // Default to TinyLlama
  static bool _isModelLoaded = false;
  static bool _isInitialized = false;

  static late final DynamicLibrary _lib;
  static late final LoadModel _loadModelFn;
  static late final RunInference _runInferenceFn;
  static late final FreeString _freeStringFn;
  static late final DownloadModel _downloadModelFn;

  static Future<String> generateText({
    required ModelConfig model,
    required String prompt,
  }) async {
    print('\n=== Generate Text ===');
    print('Requested model: ${model.name}');
    print('Current model: ${_currentModel.name}');
    print('Is model loaded: $_isModelLoaded');
    
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
      final resultPtr = _runInferenceFn(input);
      final response = resultPtr.toDartString();
      
      calloc.free(input);
      _freeStringFn(resultPtr);
      
      return response;
    } catch (e) {
      print('Error during inference: $e');
      throw Exception('Failed to generate response: $e');
    }
  }

  // Private methods
  static Future<void> _switchModel(ModelConfig model) async {
    print('\n=== Switching Model ===');
    print('Target model: ${model.name}');
    print('Required RAM: ${model.minRamMb}MB');
    if (model.description.isNotEmpty) {
      print('Description: ${model.description}');
    }
    
    await _initializeIfNeeded();
    
    try {
      final modelNamePtr = model.name.toNativeUtf8();
      final downloadPtr = _downloadModelFn(modelNamePtr);
      final downloadResult = downloadPtr.toDartString();
      print('Download result: $downloadResult');
      _freeStringFn(downloadPtr);

      final loadResultPtr = _loadModelFn(modelNamePtr);
      final loadResult = loadResultPtr.toDartString();
      print('Load result: $loadResult');
      
      calloc.free(modelNamePtr);
      _freeStringFn(loadResultPtr);
      
      print('Model switch complete');
    } catch (e) {
      print('Error during model switch: $e');
      rethrow;
    }
  }

  static Future<void> _initializeIfNeeded() async {
    if (_isInitialized) return;

    _lib = DynamicLibrary.open(_getLibraryPath());
    
    _loadModelFn = _lib.lookupFunction<LoadModelC, LoadModel>('load_model_c');
    _runInferenceFn = _lib.lookupFunction<RunInferenceC, RunInference>('run_inference_c');
    _freeStringFn = _lib.lookupFunction<FreeStringC, FreeString>('free_string_c');
    _downloadModelFn = _lib.lookupFunction<DownloadModelC, DownloadModel>('download_model_c');

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

  /// Downloads a model if not already present
  static Future<void> downloadModel(ModelConfig model) async {
    await _initializeIfNeeded();
    final modelNamePtr = model.name.toNativeUtf8();
    final resultPtr = _downloadModelFn(modelNamePtr);
    final result = resultPtr.toDartString();
    print('Download result for ${model.name}: $result');
    
    calloc.free(modelNamePtr);
    _freeStringFn(resultPtr);
  }

  /// Pre-loads a model into memory
  static Future<void> loadModel(ModelConfig model) async {
    await _initializeIfNeeded();
    final modelNamePtr = model.name.toNativeUtf8();
    final resultPtr = _loadModelFn(modelNamePtr);
    final result = resultPtr.toDartString();
    print('Load result for ${model.name}: $result');
    
    calloc.free(modelNamePtr);
    _freeStringFn(resultPtr);
  }

  /// Runs inference on the loaded model
  static Future<String> runInference(String prompt) async {
    final promptPtr = prompt.toNativeUtf8();
    final resultPtr = _runInferenceFn(promptPtr);
    final response = resultPtr.toDartString();
    
    calloc.free(promptPtr);
    _freeStringFn(resultPtr);
    
    return response;
  }
}
