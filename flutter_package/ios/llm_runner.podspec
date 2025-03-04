Pod::Spec.new do |s|
  # ... other configurations ...
  
  s.ios.deployment_target = '11.0'
  s.static_framework = true
  
  s.vendored_frameworks = 'Frameworks/libllm_runner.a'
  s.library = 'c++'
  
  # If you need any system frameworks
  s.frameworks = 'Foundation'
end 