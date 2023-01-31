pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::{game_manager_client::GameManagerClient, CoachRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GameManagerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CoachRequest {
        context: "Tonic".into(),
    });

    let response = client.get_training_menu(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
