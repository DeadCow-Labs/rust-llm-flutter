// This should be the ONLY place ModelConfig is defined
class ModelConfig {
  final String name;
  final int minRamMb;
  final String description;
  final Map<String, String> metadata;
  final bool requiresAuth;

  const ModelConfig({
    required this.name,
    required this.minRamMb,
    this.description = '',
    this.metadata = const {},
    this.requiresAuth = false,
  });

  @override
  bool operator ==(Object other) =>
    identical(this, other) ||
    other is ModelConfig &&
    runtimeType == other.runtimeType &&
    name == other.name;

  @override
  int get hashCode => name.hashCode;
}

/// Pre-configured models that are known to work well
class Models {
  // Small Models (4GB+ RAM)
  static final tinyllama = ModelConfig(
    name: 'TinyLlama/TinyLlama-1.1B-Chat-v0.6',
    minRamMb: 4096,
    description: 'Fast, lightweight model good for basic tasks',
  );

  static final phi15 = ModelConfig(
    name: 'microsoft/phi-1_5',
    minRamMb: 4096,
    description: 'Good at coding and reasoning tasks',
  );

  static final gemma2b = ModelConfig(
    name: 'google/gemma-2b-it',
    minRamMb: 4096,
    description: "Google lightweight instruction-following model",
  );

  // Medium Models (6GB+ RAM)
  static final llama32_3b = ModelConfig(
    name: 'meta-llama/Llama-3.2-3b-instruct',
    minRamMb: 6144,
    description: 'Latest Llama 3.2 instruction model',
    requiresAuth: true,  // Requires HF token
  );

  static final mistral7b = ModelConfig(
    name: 'mistralai/Mistral-7B-Instruct-v0.3',
    minRamMb: 6144,
    description: 'Powerful open-source model with good performance',
    requiresAuth: true,  // Requires HF token
  );

  // Large Models (8GB+ RAM)
  static final qwen7b = ModelConfig(
    name: 'Qwen/Qwen2.5-7B-Chat',
    minRamMb: 8192,
    description: 'High-quality multilingual model',
    requiresAuth: true,  // Requires HF token
  );

  /// Create a custom model configuration
  static ModelConfig custom({
    required String name,
    required int minRamMb,
    String description = '',
    Map<String, String> metadata = const {},
    bool requiresAuth = false,
  }) {
    return ModelConfig(
      name: name,
      minRamMb: minRamMb,
      description: description,
      metadata: metadata,
      requiresAuth: requiresAuth,
    );
  }
} 