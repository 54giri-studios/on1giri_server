use rocket_ws as ws;
use rocket::futures::{SinkExt, StreamExt};
use ws::Message;

async fn message_handler(
    stream: &mut ws::stream::DuplexStream,
    message: Result<Message, ws::result::Error>
) -> Result<(), ws::result::Error> {

    let message = message?.into_text()?;

    stream.send(message.into()).await?;

    Ok(())
}


#[get("/echo")]
pub fn echo_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            let _ = message_handler(&mut stream, message).await;
        }
        Ok(())
    }))
}
