extern crate gptman;
use std::fs;

fn main(){
   let mut f = fs::File::open("/dev/sda")
      .expect("Cannot open device");
   let mut gpt = gptman::GPT::find_from(&mut f)
      .expect("could not find GPT");

   println!("Disk GUID: {:?}", gpt.header.disk_guid);


      gpt[1] = gptman::GPTPartitionEntry {
        partition_type_guid: [0xff; 16],
        unique_parition_guid: [0xff; 16],
        starting_lba: gpt.header.first_usable_lba,
        ending_lba: gpt.header.last_usable_lba,
        attribute_bits: 0,
        partition_name: "A Robot Named Fight!".into(),
        };
    drop(f);
    let mut f = fs::OpenOptions::new().write(true).open("/dev/sda")
                .expect("Cannot open device for write");
    gpt.write_into(&mut f)
        .expect("Cannot write data into gpt");
}

