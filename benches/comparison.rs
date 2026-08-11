use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use eventsource_client::{Client, ClientBuilder};
use eventsource_stream2::Eventsource;
use launchdarkly_sdk_transport::HyperTransport;
use reqwest_eventsource::EventSource;
use reqwest_sse::EventSource as _;
use sse_reqwest_client::{RequestBuilderExt, SseRetryConfig};
use tokio_stream::StreamExt as _;

async fn eventsource_client(client: &impl Client) {
    let mut stream = client.stream();
    let mut count = 0;
    while let Some(event) = stream.try_next().await.unwrap() {
        if let eventsource_client::SSE::Event(_) = event {
            count += 1;
        }
        if count == 100 {
            break;
        }
    }
}

async fn reqwest_sse(client: &reqwest::Client) {
    let mut events = client
        .get("http://localhost:8080")
        .send()
        .await
        .unwrap()
        .events()
        .await
        .unwrap();

    let mut count = 0;
    while let Some(event) = events.next().await {
        event.unwrap();
        count += 1;
        if count == 100 {
            break;
        }
    }
}

async fn sse_reqwest_client(client: &reqwest::Client) {
    let mut stream = client
        .get("http://localhost:8080/")
        .into_event_source_builder()
        .retry_config(SseRetryConfig::disabled())
        .build();
    let mut count = 0;
    while let Some(event) = stream.next().await {
        event.unwrap();
        count += 1;
        if count == 100 {
            break;
        }
    }
}

async fn reqwest_eventsource(client: &reqwest_old::Client) {
    let mut stream = EventSource::new(client.get("http://localhost:8080")).unwrap();
    let mut count = 0;
    while let Some(event) = stream.next().await {
        if let reqwest_eventsource::Event::Message(_) = event.unwrap() {
            count += 1;
        }
        if count == 100 {
            break;
        }
    }
}

async fn sse_stream(client: &reqwest::Client) {
    let body = client
        .get("http://localhost:8080")
        .send()
        .await
        .unwrap()
        .bytes_stream();
    let mut stream = sse_stream::SseStream::from_bytes_stream(body);
    let mut count = 0;
    while let Some(event) = stream.next().await {
        event.unwrap();
        count += 1;
        if count == 100 {
            break;
        }
    }
}

async fn sseer(client: &reqwest::Client) {
    let rq = client.get("http://localhost:8080");
    let mut stream = sseer::EventSource::new(rq).unwrap();
    let mut count = 0;
    while let Some(event) = stream.next().await {
        if let sseer::reqwest::StreamEvent::Event(_) = event.unwrap() {
            count += 1;
        }
        if count == 100 {
            break;
        }
    }
}

async fn eventsource_stream2(client: &reqwest::Client) {
    let mut stream = client
        .get("http://localhost:8080")
        .send()
        .await
        .unwrap()
        .bytes_stream()
        .eventsource();
    let mut count = 0;
    while let Some(event) = stream.next().await {
        event.unwrap();
        count += 1;
        if count == 100 {
            break;
        }
    }
}

fn b(c: &mut Criterion) {
    let mut group = c.benchmark_group("sse-impls");

    let transport = HyperTransport::new().unwrap();
    let client = ClientBuilder::for_url("http://localhost:8080/")
        .unwrap()
        .build_with_transport(transport);
    group.bench_function("eventsource-client", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| eventsource_client(black_box(&client)))
    });

    let client = reqwest::Client::new();
    group.bench_function("reqwest-sse", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| reqwest_sse(black_box(&client)))
    });

    let client = reqwest::Client::new();
    group.bench_function("sse-reqwest-client", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| sse_reqwest_client(black_box(&client)))
    });

    let client = reqwest_old::Client::new();
    group.bench_function("reqwest-eventsource", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| reqwest_eventsource(black_box(&client)))
    });

    let client = reqwest::Client::new();
    group.bench_function("sse-stream", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| sse_stream(black_box(&client)))
    });

    let client = reqwest::Client::new();
    group.bench_function("sseer", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| sseer(black_box(&client)))
    });

    let client = reqwest::Client::new();
    group.bench_function("eventsource-stream2", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| eventsource_stream2(black_box(&client)))
    });
}

criterion_group!(benches, b);
criterion_main!(benches);
