## Макросы
### service
Ставится над структурой которую надо ложить в контейнер с сервисами
```
#[service(name = "zhopa")]
struct Hui {
    size: bigint
}
```
скомпилячится в 
```
struct Hui {
    size: bigint
}
impl Service for Hui {
    fn name() -> String {
        return "zhopa".to_string();
    }
}
```

