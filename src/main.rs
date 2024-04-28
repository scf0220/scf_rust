use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use futures::future::join_all;
use chrono::Local;
use std::thread;

async fn get_job(send_chan:Sender<u64>){
    let mut new_job_poll_timer = tokio::time::interval(Duration::from_secs(1));
    loop {
        new_job_poll_timer.tick().await;
        send_chan.send(1).await.unwrap();
        println!("begin sleep {:?}",Local::now());
        tokio::time::sleep(Duration::from_secs(5)).await;
        println!("end sleep {:?}",Local::now());
    }

}




async fn create_proof(mut receive_chan:Receiver<u64>){
    loop {
        let job = receive_chan.recv().await;
        if job.is_none() {
            println!("scf-log create proof: no proof {:?}",Local::now());
        }
        let job_detail=job.unwrap();
        println!("begin handle {:?} {:?}",job_detail,Local::now());
        tokio::time::sleep(Duration::from_secs(30)).await;
        println!("end handle{:?} {:?}",job_detail,Local::now());
    }

}

#[tokio::main]
async fn main() {

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
