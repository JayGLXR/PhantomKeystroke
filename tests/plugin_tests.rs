#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use std::collections::HashMap;
    
    // Mock trait for C2 Adapters (plugins)
    #[async_trait]
    trait MockC2Adapter {
        async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
        async fn receive(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>>;
        async fn send(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>>;
        fn name(&self) -> String;
        async fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    }
    
    // Null plugin implementation (echo plugin)
    struct MockNullPlugin {
        message_buffer: Vec<String>,
        initialized: bool,
    }
    
    impl MockNullPlugin {
        fn new() -> Self {
            MockNullPlugin {
                message_buffer: Vec::new(),
                initialized: false,
            }
        }
    }
    
    #[async_trait]
    impl MockC2Adapter for MockNullPlugin {
        async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.initialized = true;
            Ok(())
        }
        
        async fn receive(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
            if self.message_buffer.is_empty() {
                Ok(None)
            } else {
                Ok(Some(self.message_buffer.remove(0)))
            }
        }
        
        async fn send(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
            // Null plugin echoes back the message
            self.message_buffer.push(message.to_string());
            Ok(())
        }
        
        fn name(&self) -> String {
            "Null Plugin".to_string()
        }
        
        async fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.message_buffer.clear();
            Ok(())
        }
    }
    
    // A simple plugin manager
    struct MockPluginManager {
        plugin: Box<dyn MockC2Adapter + Send>,
    }
    
    impl MockPluginManager {
        fn new() -> Self {
            let plugin = Box::new(MockNullPlugin::new());
            MockPluginManager { plugin }
        }
        
        async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            self.plugin.initialize().await
        }
        
        fn name(&self) -> String {
            self.plugin.name()
        }
        
        async fn send(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
            self.plugin.send(message).await
        }
        
        async fn receive(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
            self.plugin.receive().await
        }
    }
    
    #[tokio::test]
    async fn test_null_plugin() {
        // Create plugin manager with null plugin
        let mut plugin_manager = MockPluginManager::new();
        
        // Initialize plugin
        plugin_manager.initialize().await.unwrap();
        
        // Test plugin name
        assert_eq!("Null Plugin", plugin_manager.name());
        
        // Test send and receive
        plugin_manager.send("test message").await.unwrap();
        let received = plugin_manager.receive().await.unwrap();
        
        // The null plugin should echo back the message
        assert_eq!(Some("test message".to_string()), received);
        
        // After reading one message, the buffer should be empty
        let empty = plugin_manager.receive().await.unwrap();
        assert_eq!(None, empty);
    }
    
    #[tokio::test]
    async fn test_multiple_messages() {
        // Create plugin manager with null plugin
        let mut plugin_manager = MockPluginManager::new();
        
        // Initialize plugin
        plugin_manager.initialize().await.unwrap();
        
        // Send multiple messages
        plugin_manager.send("message 1").await.unwrap();
        plugin_manager.send("message 2").await.unwrap();
        plugin_manager.send("message 3").await.unwrap();
        
        // Receive them in order
        assert_eq!(Some("message 1".to_string()), plugin_manager.receive().await.unwrap());
        assert_eq!(Some("message 2".to_string()), plugin_manager.receive().await.unwrap());
        assert_eq!(Some("message 3".to_string()), plugin_manager.receive().await.unwrap());
        
        // Buffer should now be empty
        assert_eq!(None, plugin_manager.receive().await.unwrap());
    }
}