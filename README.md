# mailgun-rs

> MailGun API client written in Rust.

This crate helps facilitate interacting with the MailGun API. You will need
to supply both an *API Key* and *Domain*.

[API Reference](https://documentation.mailgun.com/en/latest/api_reference.html)

## Usage

### Send a Message

```rust
use mailgun_sdk::{
    Client,
    ParamList,
    send_message::{SendMessageParam, SendMessageParamList},
};

let client = Client::new("ApiKey", "Domain");

let params = SendMessageParamList::default()
    .add(SendMessageParam::To("to@test.com"))
    .add(SendMessageParam::From("from@your-domain.com"))
    .add(SendMessageParam::Subject("Test Message"))
    .add(SendMessageParam::Html(r#"<html>
        <body>
            <h1>Test Message</h1>
        </body>
    </html>"#));

if let Err(error) = client.send_message(params) {
    eprintln!("Error: {:?}", error);
}
```

This crate does not enforce rules on sending messages. However, you should
almost always set the following when sending a message:

- **Subject**
- **To**
- **From**
- **Html** _and/or_ **Text**

**Caution:** Not all send message request parameters have been tested. If
you notice any that do not work, please feel free to create a ticket, or
create a pull a request.

### Pagination

For API calls that return a list of results, MailGun returns a `paging`
structure. The paging fields are all URLs. Instead of having to parse these,
you may use the `call` method to fetch these pages.

```rust
let mut response = client.get_bounces(GetBouncesParamList::default()).unwrap();
let mut bounces = response.items;

if bounces.len() > 0 {
    loop {
        response = client.call(&response.paging.next).unwrap();

        if response.items.len() == 0 {
            break;
        } else {
            bounces.append(&mut response.items);
        }
    }
}
```

### Further Examples

```rust
let client = Client::new("ApiKey", "Domain");

// Get all events.
let events = client.get_events(GetEventsParamList::default()).unwrap();

// Get all bounces.
let bounces = client.get_bounces(GetBouncesParamList::default()).unwrap();

// Get account stats.
let stats = client.get_stats(GetStatsParamList::default()).unwrap();
```

## Testing

Before you start testing, you will need to create a `.test.env` file in the project's root directory.

```bash
touch ./.test.env
```

The file should contain the following settings:

```
MAILGUN_API_KEY=''
MAILGUN_DOMAIN=''
```
