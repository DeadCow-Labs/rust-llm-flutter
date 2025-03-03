import 'package:flutter/material.dart';
import 'package:llm_runner/llm_runner.dart';
import 'dart:ffi';
import 'dart:io';
import 'package:path/path.dart' as path;

// Get the library name based on the platform
String _getLibraryName() {
  if (Platform.isMacOS) return 'libllm_runner.dylib';
  if (Platform.isWindows) return 'llm_runner.dll';
  return 'libllm_runner.so';  // Linux and Android
}

void main() {
  // Load the dynamic library
  final libraryPath = path.join(
    Directory.current.path,
    '../rust/target/debug',
    _getLibraryName(),
  );
  print('Looking for library at: $libraryPath');
  
  final dylib = DynamicLibrary.open(libraryPath);
  final bindings = LlmBindings(dylib);

  runApp(MyApp(bindings: bindings));
}

class MyApp extends StatelessWidget {
  final LlmBindings bindings;
  
  const MyApp({Key? key, required this.bindings}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text("LLM Runner Test")),
        body: Center(
          child: TestWidget(bindings: bindings),
        ),
      ),
    );
  }
}

class TestWidget extends StatefulWidget {
  final LlmBindings bindings;
  
  const TestWidget({Key? key, required this.bindings}) : super(key: key);

  @override
  State<TestWidget> createState() => _TestWidgetState();
}

class _TestWidgetState extends State<TestWidget> {
  String _result = 'No test run yet';

  Future<void> _runTest() async {
    try {
      setState(() => _result = 'Running test...');
      
      // Convert input string to native UTF8
      final input = "Hello, AI!".toNativeUtf8();
      
      // Run inference
      final result = widget.bindings.run_inference_c(input);
      
      // Convert result back to Dart string
      final output = result.toDartString();
      
      // Free the memory
      widget.bindings.free_string(result);
      input.free();
      
      setState(() => _result = 'Test result: $output');
    } catch (e) {
      setState(() => _result = 'Error: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        ElevatedButton(
          onPressed: _runTest,
          child: const Text('Run Test'),
        ),
        const SizedBox(height: 20),
        Text(_result),
      ],
    );
  }
}
