import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart';
import 'package:path/path.dart' as path;

typedef LoadModelC = Pointer<Utf8> Function(Pointer<Utf8>);
typedef RunInferenceC = Pointer<Utf8> Function(Pointer<Utf8>);
typedef FreeStringC = void Function(Pointer<Utf8>);
typedef FreeStringRust = Void Function(Pointer<Utf8>);

void main() {
  // Load the dynamic library from the release build
  final libraryPath = path.join(
    Directory.current.parent.parent.parent.path,
    'rust',
    'target',
    'release',  // Using release build for better performance
    'libllm_runner.dylib',
  );
  print('Looking for library at: $libraryPath');
  
  final dylib = DynamicLibrary.open(libraryPath);
  print('Library loaded successfully!');

  // Get function references
  final loadModel = dylib.lookupFunction<LoadModelC, LoadModelC>('load_model_c');
  final runInference = dylib.lookupFunction<RunInferenceC, RunInferenceC>('run_inference_c');
  final freeString = dylib.lookupFunction<FreeStringRust, FreeStringC>('free_string');

  // First load the model
  final modelName = "TinyLlama/TinyLlama-1.1B-Chat-v0.6".toNativeUtf8();
  final loadResult = loadModel(modelName);
  final loadResultString = loadResult.toDartString();
  print('Load result: $loadResultString');
  
  // Free the strings
  calloc.free(modelName);
  freeString(loadResult);

  // Then run inference
  final input = "Tell me a short joke".toNativeUtf8();
  final result = runInference(input);
  final resultString = result.toDartString();
  print('Inference result: $resultString');
  
  // Free the strings
  calloc.free(input);
  freeString(result);
}