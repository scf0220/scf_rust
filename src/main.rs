use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use futures::future::join_all;
use chrono::Local;
use std::thread;
use log::{info};
use simple_logger;
use futures::{pin_mut, FutureExt};


async fn get_job(send_chan:Sender<u64>){
    let mut new_job_poll_timer = tokio::time::interval(Duration::from_secs(1));
    loop {
        new_job_poll_timer.tick().await;
        send_chan.send(1).await.unwrap();
        info!("get_job_task: begin sleep");
        // thread::sleep(Duration::from_secs(10)); // 使用这种方式，等10s后另一个task才能接收到channel的数据
        tokio::time::sleep(Duration::from_secs(10)).await;
        info!("get_job_task: end sleep");
    }

}



async fn heart_beat_func(){
    tokio::time::sleep(Duration::from_secs(60)).await;
}
async fn execute_proof_func(){
    tokio::time::sleep(Duration::from_secs(30)).await;
}


async fn create_proof(mut receive_chan:Receiver<u64>){
    loop {
        let job = receive_chan.recv().await;
        let job_detail=job.unwrap();
        info!("create_proof_task: begin handle job_context={:?}",job_detail);
        let h_f=heart_beat_func().fuse();
        let e_f=execute_proof_func().fuse();
        pin_mut!(h_f,e_f);
        futures::select! {
        _ = h_f => info!("Heartbeat future completed"),
        _ = e_f => info!("Execute proof future completed"),
    }
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
