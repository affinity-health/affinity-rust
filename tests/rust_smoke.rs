use affinity_health_sdk::prelude::*;
use std::{io::{Read, Write}, net::TcpListener, collections::HashMap};

#[tokio::test]
async fn transport_contract() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let worker = std::thread::spawn(move || {
        let mut requests = Vec::new();
        for index in 0..2 {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).unwrap();
            let mut bytes = Vec::new();
            let mut byte = [0];
            while !bytes.ends_with(b"\r\n\r\n") { stream.read_exact(&mut byte).unwrap(); bytes.push(byte[0]); }
            let headers = String::from_utf8(bytes).unwrap();
            let length = headers.lines().find_map(|line| line.to_lowercase().strip_prefix("content-length: ").map(|s| s.parse::<usize>().unwrap())).unwrap_or(0);
            let mut body = vec![0; length];
            stream.read_exact(&mut body).unwrap();
            requests.push((headers, String::from_utf8(body).unwrap()));
            let (status, response) = if index == 0 { ("200 OK", r#"{"object":"list","data":[],"hasMore":false,"url":"/v1/orders"}"#) } else { ("422 Unprocessable Entity", r#"{"title":"Synthetic failure","status":422,"detail":"Invalid request"}"#) };
            write!(stream, "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", status, response.len(), response).unwrap();
        }
        requests
    });
    let client = ApiClient::new(ClientConfig {
        base_url: format!("http://{}", address), api_key: Some("synthetic-key".into()), max_retries: 0,
        custom_headers: HashMap::from([("Affinity-Version".into(), "2026-09-28".into())]), ..Default::default()
    }).unwrap();
    let page = client.orders.list_orders(&ListOrdersQueryRequest { limit: Some(2), starting_after: Some("ord_cursor".into()), ..Default::default() }, None).await.unwrap();
    assert!(page.data.is_empty()); assert!(!page.has_more);
    let request: CreateOrderRequest = serde_json::from_value(serde_json::json!({"practiceId":"prac_synthetic","patientId":"pat_synthetic","prescriptions":[]})).unwrap();
    assert!(client.orders.create_order(&request, Some(RequestOptions::new().additional_header("Idempotency-Key", "stable-synthetic-key"))).await.is_err());
    let requests = worker.join().unwrap();
    assert!(requests[0].0.contains("limit=2")); assert!(requests[0].0.contains("startingAfter=ord_cursor"));
    for (headers, _) in &requests {
        let h = headers.to_lowercase();
        assert!(h.contains("x-affinity-api-key: synthetic-key")); assert!(h.contains("affinity-version: 2026-09-28"));
    }
    assert!(requests[1].0.to_lowercase().contains("idempotency-key: stable-synthetic-key"));
    assert_eq!(serde_json::from_str::<serde_json::Value>(&requests[1].1).unwrap()["practiceId"], "prac_synthetic");
}
