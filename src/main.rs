use rand::prelude::*;
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

static BATSUGUN_CHART: &[[u32; 2]] = &[
    /* Fire     */ [ 2, 4],[ 2, 6],[ 2,12],[ 2,17],
    /* Water    */ [ 3, 2],[ 3, 9],[ 3,13],
    /* Grass    */ [ 4, 3],[ 4, 9],[ 4,13],
    /* Electric */ [ 5, 3],[ 5,10],
    /* Ice      */ [ 6, 4],[ 6, 9],[ 6,10],[ 6,15],
    /* Fighting */ [ 7, 1],[ 7, 6],[ 7,13],[ 7,16],[ 7,17],
    /* Poison   */ [ 8, 4],[ 8,18],
    /* Ground   */ [ 9, 2],[ 9, 5],[ 9, 8],[ 9,13],[ 9,17],
    /* Flying   */ [10, 4],[10, 7],[10,12],
    /* Psychic  */ [11, 7],[11, 8],
    /* Bug      */ [12, 4],[12,11],[12,16],
    /* Rock     */ [13, 2],[13, 6],[13,10],[13,12],
    /* Ghost    */ [14,11],[14,14],
    /* Dragon   */ [15,15],
    /* Dark     */ [16,11],[16,14],
    /* Steel    */ [17, 6],[17,13],[17,18],
    /* Fairy    */ [18, 7],[18,15],[18,16],
];

#[tonic::async_trait]
impl GameManager for MyGameManager {
    async fn get_training_menu(
        &self,
        _request: Request<CoachRequest>,
    ) -> Result<Response<CoachReply>, Status> {
        let mut types0 = Vec::new();
        let mut types1 = Vec::new();
        let mut rewsna = Vec::new();
        let mut rng = thread_rng();
        let mut indices = [
             0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
            10,11,12,13,14,15,16,17,18,19,
            20,21,22,23,24,25,26,27,28,29,
            30,31,32,33,34,35,36,37,38,39,
            40,43,44,45,46,47,48,49,50,
        ];
        indices.shuffle(&mut rng);
        for index in &indices[0..5] {
            if rng.gen_bool(0.5) {
                types0.push(BATSUGUN_CHART[*index][0]);
                types1.push(BATSUGUN_CHART[*index][1]);    
                rewsna.push(0);
            } else {
                types0.push(BATSUGUN_CHART[*index][1]);
                types1.push(BATSUGUN_CHART[*index][0]);
                rewsna.push(1);
            }
        }
        rewsna.reverse();

        let reply = hello_world::CoachReply {
            types0, types1, rewsna
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
