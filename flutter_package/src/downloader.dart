import 'package:http/http.dart' as http;
import 'dart:io';

class ModelDownloader {
  static Future<void> download(String modelName) async {
    final url = 'https://models.example.com/$modelName';
    final response = await http.get(Uri.parse(url));

    if (response.statusCode == 200) {
      File file = File('/models/$modelName');
      await file.writeAsBytes(response.bodyBytes);
    } else {
      throw Exception("Failed to download model.");
    }
  }
}
