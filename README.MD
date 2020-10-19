## Макросы (аудейтед)
### service
Ставится над структурой которую надо ложить в контейнер с сервисами
```
#[service(name = "zhopa")]
struct Ass {
    size: bigint
}
```
скомпилируется в 
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

### join
где возможно надо использовать `join` для асинхронных операций, например несколько чтений

```
impl StackServiceIf for StackService {
async fn add_to_my_stack(&self, user: User, new_stack_item: NewStackItem) -> StackItem {
       let ids = vec![Id("123".to_string())];
       let ids2 = vec![Id("456".to_string())];

       let a = self.marks_repo.find_by_ids(&ids);
       let b = self.marks_repo.find_by_ids(&ids2);
       // let b = self.marks_repo.find_by_ids(&vec![Id("456".to_string())]);
       let c = futures::join!(a, b);
```

