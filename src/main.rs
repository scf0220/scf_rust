use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use futures::future::join_all;
use chrono::Local;
use std::thread;
use log::{info, warn};
use simple_logger;


async fn get_job(send_chan:Sender<u64>){
    let mut new_job_poll_timer = tokio::time::interval(Duration::from_secs(1));
    loop {
        new_job_poll_timer.tick().await;
        send_chan.send(1).await.unwrap();
        info!("get_job_task: begin sleep");
        // thread::sleep(Duration::from_secs(5));
        tokio::time::sleep(Duration::from_secs(5)).await;
        info!("get_job_task: end sleep");
    }

}




async fn create_proof(mut receive_chan:Receiver<u64>){
    loop {
        let job = receive_chan.recv().await;
        if job.is_none() {
            info!("scf-log create proof: no proof {:?}",Local::now());
        }
        let job_detail=job.unwrap();
        info!("create_proof_task: begin handle job_context={:?}",job_detail);
        // thread::sleep(Duration::from_secs(30));
        tokio::time::sleep(Duration::from_secs(30)).await;
        info!("create_proof_task: end handle ");
    }

}

#[tokio::main]
async fn main() {
    simple_logger::init();
    let (job_send_chan, job_recv_chan): (
        Sender<u64>,
        Receiver<u64>,
    ) = channel(1);


    let mut handles = Vec::with_capacity(1);

    {
        let get_job_handle=tokio::task::spawn(async move {
            get_job(job_send_chan).await;
        });
        handles.push(get_job_handle)
    }

    {
        let create_proof_handle=tokio::task::spawn(async move {
            create_proof(job_recv_chan).await;
        });
        handles.push(create_proof_handle)
    }

    for result in join_all(handles).await {
        result.unwrap();
    }
    println!("All tasks finished.");
}
