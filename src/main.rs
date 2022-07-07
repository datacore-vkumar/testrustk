use std::{thread, time};
use nats::jetstream::{StreamConfig, StorageType, PublishOptions};

fn main() {


    let nc = nats::connect("nats:4222").unwrap();
    let js = nats::jetstream::new(nc);
    //js.delete_stream("mystreama".to_string());
    //js.delete_stream("mystream".to_string());
   /*let k = match js.add_stream(StreamConfig {
        name: "mystreami".to_string(),
        subjects : vec!["vikq".to_string(),"ansq".to_string()],
        max_bytes: 5 * 1024 * 1024 * 1024,
        storage: StorageType::Memory,
        discard: nats::jetstream::DiscardPolicy::Old,
        retention: nats::jetstream::RetentionPolicy::Interest,
        max_consumers : 1,
        ..Default::default()
    })
   {
       Ok(k) => k,
       Err(err ) => panic!("Error: {}", err)
   };
    println!("{:?}",k);

    */
    let mut n = 1;
    println!("Running man");

    // Loop while `n` is less than 101
    while n < 10 {
        js.publish("orders.recieved","Hello");
        let ten_millis = time::Duration::from_millis(5000);
        thread::sleep(ten_millis);
    }

}
