use imgen::Img;
use lambda_http::{
    ext::RequestExt,
    handler,
    lambda_runtime::{self, Context},
    Body, IntoResponse, Request, Response,
};
use serde_json::json;
use sha2::{Digest, Sha512};

type E = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), E> {
    lambda_runtime::run(handler(|event: Request, ctx: Context| {
        render_image(event, ctx)
    }))
    .await?;

    Ok(())
}

async fn render_image(event: Request, _: Context) -> Result<impl IntoResponse, E> {
    let path_parameters = event.path_parameters();
    let id = match path_parameters.get("id") {
        Some(id) => id,
        None => {
            return Ok(Response::builder()
                .status(400)
                .header("Content-Type", "application/json")
                .body(Body::Text(json!({ "error": "missing id" }).to_string()))
                .unwrap());
        }
    };

    // Get hash from ID
    let mut hasher = Sha512::new();
    hasher.update(id.as_bytes());
    let hash = hasher.finalize();

    // Create the image
    let img = Img::new(hash.as_slice().try_into().unwrap()).render();

    // Store the image as PNG in memory
    let mut buf = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    encoder
        .encode(&img, 256, 256, image::ColorType::Rgb8)
        .unwrap();

    // Return the image
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "image/png")
        .body(Body::Binary(buf))
        .unwrap())
}
