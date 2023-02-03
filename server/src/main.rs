use tonic::{transport::Server, Request, Response, Status};
use voting::{VotingRequest, VotingResponse, voting_server::{Voting, VotingServer}};

pub mod voting {
    tonic::include_proto!("voting");
}

#[derive(Debug, Default)]
pub struct VotingService {}

#[tonic::async_trait]
impl Voting for VotingService {
  async fn vote(&self, request: Request<VotingRequest>) -> Result<Response<VotingResponse>, Status> {
    let r = request.into_inner();
    match r.vote {
      0 => Ok(Response::new(voting::VotingResponse { confirmation: {
        println!("Voted up for {}", r.url);
        format!("Happy to confirm that you upvoted for {}", r.url)
      }})),
      1 => Ok(Response::new(voting::VotingResponse { confirmation: {
        println!("Voted down for {}", r.url);
        format!("Confirmation that you downvoted for {}", r.url)
      }})),
      _ => {
        println!("Invalid vote");
        Err(Status::new(tonic::Code::OutOfRange, "Invalid vote provided"))
      }
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let address = "[::1]:8080".parse().unwrap();
  let voting_service = VotingService::default();

  Server::builder().add_service(VotingServer::new(voting_service))
    .serve(address)
    .await?;
  Ok(())

}
