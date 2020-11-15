use actix::prelude::{Context as ActCtx, Message};
use actix::{Actor, Handler, StreamHandler};
use actix_cors::Cors;
use actix_rt::time::Instant;
use actix_web::http::{header, Method};
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result as ActixResult};
use actix_web_actors::ws;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::static_assertions::_core::time::Duration;
use async_graphql::SimpleObject;
use async_graphql::{Context, EmptyMutation, Error, Object, Schema, Subscription};
use async_graphql_actix_web::{Request, Response, WSSubscription};
use futures::channel::mpsc;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::task::{Context as FutContext, Poll};
use futures::{io, Stream, StreamExt};
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use slab::Slab;
use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Mutex;
use actix_web::Error as ActixError;

#[derive(Clone, SimpleObject)]
pub struct Change {
    chto: String,
}

// static USER_GROUPS_SUBSCRIBERS: Lazy<Mutex<FxHashMap<usize, Box<dyn Any + Send>>>> =
// static SENDERS: Lazy<Mutex<FxHashMap<String, Senders>>> = Lazy::new(Default::default);
static SENDERS: Lazy<Mutex<FxHashMap<String, Slab<UnboundedSender<Change>>>>> =
    Lazy::new(Default::default);

struct Senders(Slab<UnboundedSender<Change>>);

// usize - индес куда в slab вставлен, чтобы потом в дропе удалять из slab
struct Sstream(String, usize, UnboundedReceiver<Change>);

impl Drop for Sstream {
    fn drop(&mut self) {
        let mut map = SENDERS.lock().unwrap();
        let senders = map
            .entry(self.0.clone())
            .or_insert_with(|| Default::default())
            .remove(self.1);
        // println!("drp");
        // with_senders::<T, _, _>(|senders| senders.0.remove(self.0));
    }
}

impl Stream for Sstream {
    type Item = Change;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut FutContext<'_>) -> Poll<Option<Self::Item>> {
        self.2.poll_next_unpin(cx)
    }
}

/// A simple broker based on memory
pub struct Broker;

impl Broker {
    /// Publish a message that all subscription streams can receive.
    pub fn publish(user_id: String, msg: Change) {
        let mut map = SENDERS.lock().unwrap();

        let senders = map.entry(user_id).or_insert_with(|| Default::default());

        for (_, sender) in senders.iter_mut() {
            sender.start_send(msg.clone()).ok();
        }
    }

    /// Subscribe to the message of the specified type and returns a `Stream`.
    pub fn subscribe(user_id: String) -> impl Stream<Item = Change> {
        let mut map = SENDERS.lock().unwrap();

        // сендеры для user_id
        let senders = map
            .entry(user_id.clone())
            .or_insert_with(|| Default::default());

        let (tx, rx) = mpsc::unbounded();
        let id = senders.insert(tx);
        Sstream(user_id, id, rx)
    }
}

pub type Root = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;
pub struct QueryRoot;
pub struct SubscriptionRoot;

#[Object]
impl QueryRoot {
    async fn kuk(&self, ctx: &Context<'_>) -> String {
        "kuk".to_string()
    }

    async fn test(&self, ctx: &Context<'_>) -> String {
        Broker::publish(
            "100".to_string(),
            Change {
                chto: "Vot tak".to_string(),
            },
        );

        "test".to_string()
    }
}

pub async fn index(schema: web::Data<Root>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

async fn index_ws(
    schema: web::Data<Root>,
    req: HttpRequest,
    payload: web::Payload,
) -> ActixResult<HttpResponse> {
    println!("1!!!");

    ws::start_with_protocols(
        WSSubscription::new(Schema::clone(&*schema)),
        &["graphql-ws"],
        &req,
        payload,
    )
}

type Chan = (
    futures::channel::mpsc::Sender<i32>,
    futures::channel::mpsc::Receiver<i32>,
);

#[Subscription]
impl SubscriptionRoot {
    // async fn once(&self, ctx: &Context<'_>) -> impl Stream<Item = Change> {
    // async fn once(&self, ctx: &Context<'_>) -> impl Stream<Item = Result<i32, Error>> {
    async fn once(&self, ctx: &Context<'_>) -> impl Stream<Item = i32> {
        // actix::clock::interval_at(Instant::now(), Duration::from_secs(4))

        // futures::stream::once(futures::future::ok(1))

        // futures::

        // Broker::subscribe("100".to_string())

        // let n = ctx.to_owned();
        // let nn : &mut Chan = n.data_unchecked();
        // &nn.1.borrow_mut()

        // &nn.1
        // let nn = nn.borrow_mut();

        // async fn once(&self, ctx: &Context<'_>) -> impl Stream<Item = i32> {
        // let chan: &Chan = ctx.data_unchecked::<Chan>();

        // chan.1.poll_next()

        // let cc = chan.1;
        // &chan.1.into()
        // futures::stream::once(futures::future::ok(1))
        // futures::
        let mut a = 0;
        actix::clock::interval_at(Instant::now(), Duration::from_secs(1)).map(move |_| {a += 1; a})
    }
}

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        println!("{:#?}", &msg);

        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index_webs(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, ActixError> {
    let resp = ws::start(MyWs, &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_rt::main]
async fn main() -> Result<(), io::Error> {
    let c: Chan = futures::channel::mpsc::channel(10);

    let schema = Schema::build(QueryRoot, EmptyMutation, SubscriptionRoot)
        .data(c)
        .finish();

    println!("Playground: http://localhost:8081");

    HttpServer::new(move || {
        let cors = Cors::new()
            .allowed_origin("*")
            // .allowed_origin(&allowed_origin)
            // .allowed_origin("all")
            .allowed_origin("http://127.0.0.1:8080")
            // .allowed_origin("http://127.0.0.1:9988")
            // .allowed_origin("http://127.0.0.1:3000")
            .allowed_methods(vec![Method::GET, Method::OPTIONS, Method::POST])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .finish();

        App::new()
            .wrap(cors)
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .service(web::resource("/ws/").guard(guard::Get()).to(index_webs))
    })
    .bind("127.0.0.1:8080")?
    // .route("/ws/", web::get().to(index_webs))
    .run()
    .await
}
