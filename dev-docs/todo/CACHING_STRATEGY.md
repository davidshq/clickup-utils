# ClickUp CLI - Caching Strategy & Implementation

## 📋 Overview

This document outlines the caching strategy for the ClickUp CLI, focusing on intelligent cache invalidation and seamless integration with the existing repository pattern. The caching layer will be implemented as a decorator pattern around the current repository to provide transparent performance improvements.

## 🏗️ Architecture

### Current Flow
```
Command -> Repository -> API -> HTTP
```

### With Caching Layer
```
Command -> CachedRepository -> Repository -> API -> HTTP
```

## 🔧 Implementation Strategy

### 1. **Cache Layer Components**

```rust
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct CachedValue<T> {
    data: T,
    expires_at: Instant,
    etag: Option<String>, // HTTP ETag for cache validation
}

impl<T> CachedValue<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            expires_at: Instant::now() + ttl,
            etag: None,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
}

pub struct CacheManager {
    cache: Arc<RwLock<HashMap<String, CachedValue<serde_json::Value>>>>,
    default_ttl: Duration,
}

pub struct CachedClickUpRepository {
    inner: Box<dyn ClickUpRepository>,
    cache: CacheManager,
}
```

### 2. **Cache Configuration**

```rust
#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub enabled: bool,
    pub default_ttl: Duration,
    pub max_size: usize,
    pub workspace_ttl: Duration,
    pub space_ttl: Duration,
    pub list_ttl: Duration,
    pub task_ttl: Duration,
    pub task_list_ttl: Duration,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_ttl: Duration::from_secs(60),    // 1 minute (shorter for multi-client safety)
            max_size: 1000,
            workspace_ttl: Duration::from_secs(300),  // 5 minutes (shorter for multi-client safety)
            space_ttl: Duration::from_secs(180),      // 3 minutes (shorter for multi-client safety)
            list_ttl: Duration::from_secs(120),       // 2 minutes (shorter for multi-client safety)
            task_ttl: Duration::from_secs(60),        // 1 minute (shorter for multi-client safety)
            task_list_ttl: Duration::from_secs(30),   // 30 seconds (very short for multi-client safety)
        }
    }
}
```

## 🎯 Cache Invalidation Strategies

### 1. **Time-Based Expiration (TTL)**

```rust
impl CachedClickUpRepository {
    async fn get_tasks(&self, list_id: &str) -> Result<TasksResponse, ClickUpError> {
        let cache_key = format!("tasks:list:{}", list_id);
        
        // Check cache first
        if let Some(cached) = self.cache.get(&cache_key).await {
            if !cached.is_expired() {
                return Ok(serde_json::from_value(cached.data.clone())?);
            }
        }
        
        // Fetch fresh data
        let tasks = self.inner.get_tasks(list_id).await?;
        
        // Cache with appropriate TTL
        let ttl = Duration::from_secs(300); // 5 minutes for task lists
        self.cache.set(&cache_key, &tasks, ttl).await;
        
        Ok(tasks)
    }
}
```

### 2. **Event-Based Invalidation**

```rust
impl CachedClickUpRepository {
    async fn create_task(&self, list_id: &str, task: CreateTaskRequest) -> Result<Task, ClickUpError> {
        // Create the task
        let new_task = self.inner.create_task(list_id, task).await?;
        
        // Invalidate related caches
        self.invalidate_task_caches(list_id).await;
        
        Ok(new_task)
    }
    
    async fn invalidate_task_caches(&self, list_id: &str) {
        // Invalidate task list cache
        self.cache.remove(&format!("tasks:list:{}", list_id)).await;
        
        // Invalidate workspace cache (tasks affect workspace stats)
        self.cache.remove_pattern("workspace:*").await;
        
        // Invalidate space cache
        self.cache.remove_pattern("space:*").await;
    }
}
```

### 3. **ETag-Based Validation**

```rust
impl CachedClickUpRepository {
    async fn get_task(&self, task_id: &str) -> Result<Task, ClickUpError> {
        let cache_key = format!("task:{}", task_id);
        
        if let Some(cached) = self.cache.get(&cache_key).await {
            // Check if we have a recent ETag
            if let Some(etag) = &cached.etag {
                // Make conditional request with ETag
                match self.inner.get_task_with_etag(task_id, etag).await {
                    Ok(task) => return Ok(task),
                    Err(ClickUpError::NotModified) => {
                        // Server says data hasn't changed, use cache
                        return Ok(serde_json::from_value(cached.data.clone())?);
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        
        // Fetch fresh data
        let task = self.inner.get_task(task_id).await?;
        
        // Cache with ETag
        let ttl = Duration::from_secs(600); // 10 minutes for individual tasks
        self.cache.set_with_etag(&cache_key, &task, ttl, &task.etag).await;
        
        Ok(task)
    }
}
```

## 🔑 Cache Key Strategy

```rust
impl CacheManager {
    fn generate_cache_key(&self, operation: &str, params: &[(&str, &str)]) -> String {
        let mut key = operation.to_string();
        for (k, v) in params {
            key.push_str(&format!(":{}={}", k, v));
        }
        key
    }
    
    async fn get_tasks_cache_key(&self, list_id: &str, filters: Option<&str>) -> String {
        let mut params = vec![("list_id", list_id)];
        if let Some(filter) = filters {
            params.push(("filter", filter));
        }
        self.generate_cache_key("tasks", &params)
    }
}
```

## 📊 Cache Invalidation Rules

### Read Operations (Cache-Friendly)
- `get_workspaces()` - TTL: 5 minutes (with ETag validation)
- `get_spaces()` - TTL: 3 minutes (with ETag validation)
- `get_lists()` - TTL: 2 minutes (with ETag validation)
- `get_tasks()` - TTL: 30 seconds (with ETag validation)
- `get_task()` - TTL: 1 minute (with ETag validation)

### Write Operations (Invalidate Related Caches)

```rust
impl CachedClickUpRepository {
    async fn create_task(&self, list_id: &str, task: CreateTaskRequest) -> Result<Task, ClickUpError> {
        let result = self.inner.create_task(list_id, task).await?;
        
        // Invalidate related caches
        self.cache.remove_pattern(&format!("tasks:list:{}", list_id)).await;
        self.cache.remove_pattern("workspace:*").await;
        self.cache.remove_pattern("space:*").await;
        
        Ok(result)
    }
    
    async fn update_task(&self, task_id: &str, task: UpdateTaskRequest) -> Result<Task, ClickUpError> {
        let result = self.inner.update_task(task_id, task).await?;
        
        // Invalidate specific task and related caches
        self.cache.remove(&format!("task:{}", task_id)).await;
        self.cache.remove_pattern("tasks:list:*").await;
        self.cache.remove_pattern("workspace:*").await;
        
        Ok(result)
    }
    
    async fn delete_task(&self, task_id: &str) -> Result<(), ClickUpError> {
        let result = self.inner.delete_task(task_id).await?;
        
        // Invalidate all task-related caches
        self.cache.remove(&format!("task:{}", task_id)).await;
        self.cache.remove_pattern("tasks:list:*").await;
        self.cache.remove_pattern("workspace:*").await;
        
        Ok(result)
    }
}
```

## 🔧 Integration with Repository Factory

```rust
impl RepositoryFactory {
    pub fn create_with_cache(config: &Config) -> Result<Box<dyn ClickUpRepository>, ClickUpError> {
        let api = ClickUpApi::new(config.clone())?;
        let inner_repo = ClickUpApiRepository::new(api);
        
        if config.cache.enabled {
            let cache_manager = CacheManager::new(config.cache.clone());
            let cached_repo = CachedClickUpRepository::new(Box::new(inner_repo), cache_manager);
            Ok(Box::new(cached_repo))
        } else {
            Ok(Box::new(inner_repo))
        }
    }
}
```

## 📈 Cache Monitoring

```rust
impl CacheManager {
    pub async fn get_with_metrics(&self, key: &str) -> Option<CachedValue<serde_json::Value>> {
        let result = self.get(key).await;
        
        match result {
            Some(_) => {
                self.metrics.cache_hits.fetch_add(1, Ordering::Relaxed);
            }
            None => {
                self.metrics.cache_misses.fetch_add(1, Ordering::Relaxed);
            }
        }
        
        result
    }
}
```

## 🎯 Cache Invalidation Summary

The cache determines whether to use fresh data or cached data through:

1. **TTL Expiration**: Automatic expiration based on time (shorter TTLs for multi-client safety)
2. **Event-Based Invalidation**: When related data is modified by this client
3. **ETag Validation**: HTTP ETags for conditional requests (primary strategy for multi-client safety)
4. **Pattern-Based Invalidation**: Remove related cache entries when data changes
5. **Multi-Client Awareness**: Shorter TTLs and aggressive ETag usage for data that can be modified by other clients

## 🔄 Multi-Client Cache Strategy

### **The Multi-Client Challenge**

Since ClickUp is a multi-user platform, other clients (web UI, mobile apps, other CLI instances) can modify data simultaneously. This means our cache can become stale even within our TTL windows.

### **Multi-Client Safe Caching Strategy**

```rust
impl CachedClickUpRepository {
    async fn get_tasks(&self, list_id: &str) -> Result<TasksResponse, ClickUpError> {
        let cache_key = format!("tasks:list:{}", list_id);
        
        // Always check ETag first for multi-client safety
        if let Some(cached) = self.cache.get(&cache_key).await {
            if let Some(etag) = &cached.etag {
                // Make conditional request with ETag
                match self.inner.get_tasks_with_etag(list_id, etag).await {
                    Ok(tasks) => return Ok(tasks),
                    Err(ClickUpError::NotModified) => {
                        // Server confirms data hasn't changed, use cache
                        return Ok(serde_json::from_value(cached.data.clone())?);
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        
        // Fetch fresh data
        let tasks = self.inner.get_tasks(list_id).await?;
        
        // Cache with very short TTL and ETag for multi-client safety
        let ttl = Duration::from_secs(30); // 30 seconds for task lists
        self.cache.set_with_etag(&cache_key, &tasks, ttl, &tasks.etag).await;
        
        Ok(tasks)
    }
}
```

### **ETag-First Strategy**

For multi-client safety, we prioritize ETag validation over TTL:

```rust
impl CacheManager {
    async fn get_with_etag_priority(&self, key: &str, etag: &str) -> Option<CachedValue<serde_json::Value>> {
        // Always validate with ETag first, regardless of TTL
        if let Some(cached) = self.cache.get(key).await {
            if let Some(cached_etag) = &cached.etag {
                if cached_etag == etag {
                    return Some(cached);
                }
            }
        }
        None
    }
}
```

### **Conservative TTL Strategy**

Since we can't trust that data hasn't changed from other clients, we use very conservative TTLs:

- **Task lists**: 30 seconds (highly volatile)
- **Individual tasks**: 1 minute (moderately volatile)
- **Lists**: 2 minutes (less volatile)
- **Spaces**: 3 minutes (less volatile)
- **Workspaces**: 5 minutes (least volatile)

### **ETag Validation Priority**

1. **Always check ETag first** before using cached data
2. **Fall back to TTL** only if ETag is not available
3. **Aggressive cache invalidation** when data is modified by this client
4. **Short TTLs** as a safety net for ETag failures

## ✅ Benefits

1. **Transparent**: Commands don't need to change - they just use the repository as before
2. **Multi-Client Safe**: ETag-first strategy ensures data freshness in multi-user environments
3. **Smart Invalidation**: Automatically invalidates related caches when data changes
4. **Configurable**: Can be enabled/disabled and TTLs can be adjusted
5. **Performance**: Reduces API calls by 40-60% for read-heavy operations (conservative estimate for multi-client safety)
6. **Consistent**: Uses the same repository interface, just with caching
7. **Conservative**: Short TTLs and aggressive ETag usage prevent stale data issues

## ⚖️ Multi-Client Trade-offs

### **Conservative Approach Benefits**
- **Data Freshness**: ETag validation ensures we always get the latest data
- **No Stale Data**: Short TTLs prevent long-term cache staleness
- **Reliable**: Works correctly in multi-user environments

### **Conservative Approach Costs**
- **Reduced Cache Hit Rate**: Shorter TTLs mean fewer cache hits
- **More API Calls**: ETag validation requires more conditional requests
- **Lower Performance Gain**: 40-60% vs 60-80% API call reduction

### **Alternative Approaches Considered**
1. **No Caching**: Always fetch fresh data (100% reliable, 0% performance gain)
2. **Aggressive Caching**: Longer TTLs (higher performance, risk of stale data)
3. **Smart Caching**: Conservative TTLs + ETag validation (balanced approach - chosen)

## 🚀 Implementation Roadmap

### Phase 1: Core Caching Infrastructure (1-2 weeks)
- [ ] Implement `CacheManager` with TTL support
- [ ] Create `CachedClickUpRepository` decorator
- [ ] Add cache configuration to `Config`
- [ ] Update `RepositoryFactory` to support cached repositories

### Phase 2: Smart Invalidation (1-2 weeks)
- [ ] Implement event-based cache invalidation
- [ ] Add pattern-based cache removal
- [ ] Integrate ETag support for conditional requests
- [ ] Add cache monitoring and metrics

### Phase 3: Performance Optimization (1 week)
- [ ] Fine-tune TTL values based on usage patterns
- [ ] Add cache size limits and eviction policies
- [ ] Implement cache warming for frequently accessed data
- [ ] Add cache statistics and monitoring

---

*Last updated: July 15, 2025*
*Status: Planned Implementation* 