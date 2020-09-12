use crate::handlers::{Context};

pub struct Query {}

#[juniper::graphql_object(Context = Context)]
impl Query {
    pub async fn stack(_ctx: &Context) -> Vec<String> {

        // let date_logger: &dyn DateLogger = module.resolve_ref();
        // let service: Box<dyn Service> = context.container.provide().unwrap();
        // println!("-{}-", service.get_double());



        // let service: Box<dyn Service> = context.m.
        // let mm = Arc::clone(&context.m).provider();
        // service.get_double();

        // let m_a: Arc<ExampleModule> = Arc::clone(&context.m);
        // let service: Box<dyn Service> = m_a.provide().unwrap();
        // service.get_double();

        // context.container.service()

        vec![
            "1".to_string(),
            "2".to_string()
        ]
    }

    pub async fn api_version(_ctx: &Context) -> String {
        "1".to_string()
    }
}
