use crate::DiscoveyResult;

#[derive(Debug)]
pub enum KillResult{
    WaitingToBeKilled,
    Killed,
    NotKilled(String),
}

#[derive(Debug)]
pub struct KillRequest{
    pub request: DiscoveyResult,
    pub result: KillResult

}

impl KillRequest{
    pub fn new(request: DiscoveyResult) -> KillRequest{
        KillRequest{
            request,
            result: KillResult::WaitingToBeKilled,
        }
    }
}

pub struct Killer{

}

impl Killer{
    pub fn kill(requests: Vec<DiscoveyResult>) -> Vec<KillRequest>{
        let mut results = vec![];
        for request in requests{
            // Delete entire directory
            let io_result = std::fs::remove_dir_all(request.url.clone());
            match io_result{
                Ok(_) => {
                    results.push(KillRequest{
                        request: request,
                        result: KillResult::Killed
                    });
                    continue;
                },
                Err(e) => {
                    results.push(KillRequest{
                        request: request,
                        result: KillResult::NotKilled(e.to_string())
                    });
                }
            }
            
        }
        results
       
    }

}