use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::{
    game_manager_server::{GameManager, GameManagerServer},
    CoachReply, CoachRequest,
};

#[derive(Default)]
pub struct MyGameManager {}

#[tonic::async_trait]
impl GameManager for MyGameManager {
    async fn get_training_menu(
        &self,
        _request: Request<CoachRequest>,
    ) -> Result<Response<CoachReply>, Status> {
        let reply = hello_world::CoachReply {
            types_atk: [1, 2, 3, 4, 5].to_vec(),
            types_def: [6, 7, 8, 9, 10].to_vec()
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let gm = MyGameManager::default();

    println!("GameManagerServer listening on {}", addr);

    Server::builder()
        .add_service(GameManagerServer::new(gm))
        .serve(addr)
        .await?;

    Ok(())
}
