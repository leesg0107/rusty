use tokio::sync::mpsc;
use tokio::time::{sleep,Duration};

#[tokio::main]
async fn main(){
    let (tx,mut rx)=mpsc::channel(32);

    let producer=tokio::spawn(async move{
        let mut i=0;
        loop{
            if tx.send(i).await.is_err(){
                break;
            }
            i+=1;
            sleep(Duration::from_millis(100)).await;
        }
    });

    let consumer=tokio::spawn(async move{
        while let Some(v)=rx.recv().await{
            println!("Received: {}",v);
        }
    });

    let _=tokio::try_join!(producer,consumer);
}
