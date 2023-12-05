use std::task::{Context, Poll};
use futures::stream::{Stream, StreamExt};
use std::pin::Pin;

#[derive(Debug)]
struct Message {
    role: ChatRole,
    content: String,
}

#[derive(Debug)]
enum ChatRole{
    User,
    Assistant,
}

struct MessageBuffer {
    messages: Vec<Message>,
}

impl Stream for MessageBuffer {
    type Item = Message;

    fn poll_next(
            mut self: Pin<&mut Self>, 
            cx: &mut Context<'_>
        ) -> Poll<Option<Self::Item>> {
        
        if !self.messages.is_empty() {
           std::thread::sleep(std::time::Duration::from_secs(1));
            Poll::Ready(Some(self.messages.remove(0)))
        } else {
            Poll::Ready(None)
        }
        // Poll::Ready(None)
    }
}

#[tokio::test]
async fn test_stream() {
    let mut buffer = MessageBuffer {
        messages: vec![
            Message {
                role: ChatRole::User,
                content: "What is Rust?".to_string(),
            },
            Message {
                role: ChatRole::Assistant,
                content: "Rust is a programming language".to_string(),
            },
            Message {
                role: ChatRole::User,
                content: "Briefly introduce Rust".to_string(),
            },
            Message {
                role: ChatRole::Assistant,
                content: "Rust is a multi-paradigm programming language designed for performance and safety, especially safe concurrency. Rust is syntactically similar to C++, but can guarantee memory safety by using a borrow checker to validate references. Rust achieves memory safety without garbage collection, and reference counting is optional. Rust was originally designed by Graydon Hoare at Mozilla Research, with contributions from Dave Herman, Brendan Eich, and others. The designers refined the language while writing the Servo layout or browser engine, and the Rust compiler. The compiler is free and open-source software dual-licensed under the MIT License and Apache License 2.0".to_string(),
            },
        ],
    };

    let mut i = 0;
    while let Some(message) = buffer.next().await {
        println!("{:#?}", message);
        i += 1;
    }

}