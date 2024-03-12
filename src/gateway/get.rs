use rocket_ws as ws;
use rocket::futures::{SinkExt, StreamExt};
use ws::Message;

async fn message_handler(
    stream: &mut ws::stream::DuplexStream,
    message: Result<Message, ws::result::Error>
) -> Result<(), ws::result::Error> {
    stream.send("Message received".into()).await?;

    let message = message?.into_text()?;


    Ok(())
}


#[get("/echo")]
pub fn echo_channel(ws: ws::WebSocket) -> ws::Channel<'static> {
    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            message_handler(&mut stream, message).await;
        }
        Ok(())
    }))
}