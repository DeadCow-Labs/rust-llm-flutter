import 'dart:ffi' as ffi;
import 'package:path/path.dart' as path;
import 'dart:io' show Platform, Directory;
import 'package:ffi/ffi.dart';

String _getLibraryName() {
  if (Platform.isMacOS) return 'libllm_runner.dylib';
  if (Platform.isWindows) return 'llm_runner.dll';
  return 'libllm_runner.so';
}

void main() {
  final currentDir = Directory.current.path;
  final parentDir = path.dirname(currentDir);
  final flutterPackageDir = path.dirname(parentDir);
  final projectDir = path.dirname(flutterPackageDir);
  
  final libraryPath = path.join(
    projectDir,
    'rust',
    'target/debug',
    _getLibraryName(),
  );
  
  print('Looking for library at: $libraryPath');
  
  try {
    final dylib = ffi.DynamicLibrary.open(libraryPath);
    print('Library loaded successfully!');
    
    // Use correct FFI types
    final runInference = dylib.lookupFunction<
      ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Char>),
      ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Char>)
    >('run_inference_c');
    
    // Convert input string to C string
    final input = "Hello, AI!".toNativeUtf8();
    
    // Run inference
    final result = runInference(input.cast<ffi.Char>());
    
    // Convert result back to Dart string
    final output = result.cast<Utf8>().toDartString();
    print('Inference result: $output');
    
    // Free memory
    calloc.free(input);
    calloc.free(result);
    
  } catch (e) {
    print('Error: $e');
  }
}