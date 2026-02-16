use std::sync::RwLock;

static REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
static GET_REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
static POST_REQUEST_COUNT: RwLock<u32> = RwLock::new(0);
static DELETE_REQUEST_COUNT: RwLock<u32> = RwLock::new(0);

enum Request {
    Get { endpoint: String },
    Post { endpoint: String, payload_size: u32 },
    Delete(u32),
}

fn handle_request(req: &Request) {
    {
        let mut total = REQUEST_COUNT.write().unwrap();
        *total += 1;
    }

    match req {
        Request::Get { endpoint } => {
            let mut count = GET_REQUEST_COUNT.write().unwrap();
            *count += 1;
            println!("GET request received for endpoint: {}", endpoint);
        }
        Request::Post {
            endpoint,
            payload_size,
        } => {
            let mut count = POST_REQUEST_COUNT.write().unwrap();
            *count += 1;
            println!(
                "POST request to {} with payload size {} bytes",
                endpoint, payload_size
            );
        }
        Request::Delete(id) => {
            let mut count = DELETE_REQUEST_COUNT.write().unwrap();
            *count += 1;
            println!("DELETE request for resource with id {}", id);
        }
    }
}

pub fn start() {
    let r1 = Request::Get {
        endpoint: "/users".to_string(),
    };

    let r2 = Request::Post {
        endpoint: "/login".to_string(),
        payload_size: 512,
    };

    let r3 = Request::Delete(42);

    handle_request(&r1);
    handle_request(&r1);
    handle_request(&r1);
    handle_request(&r2);
    handle_request(&r3);

    let get_cnt = *GET_REQUEST_COUNT.read().unwrap();
    let post_cnt = *POST_REQUEST_COUNT.read().unwrap();
    let delete_cnt = *DELETE_REQUEST_COUNT.read().unwrap();
    let total = *REQUEST_COUNT.read().unwrap();

    println!("Total GET requests processed: {}", get_cnt);
    println!("Total POST requests processed: {}", post_cnt);
    println!("Total DELETE requests processed: {}", delete_cnt);
    println!("Total requests processed: {}", total);
}
