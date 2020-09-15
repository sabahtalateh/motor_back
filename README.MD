## Макросы
### service
Ставится над структурой которую надо ложить в контейнер с сервисами
```
#[service(name = "zhopa")]
struct Ass {
    size: bigint
}
```
скомпилячится в 
```
struct Ass {
    size: bigint
}
impl Service for Ass {
    fn name() -> String {
        return "zhopa".to_string();
    }
}
```

