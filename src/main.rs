extern crate serde;
extern crate serde_json;
use std::process::Command;
use std::{thread, time};


fn main() {

    println!("Partition Test\n");
    read_part();
    let cnt = get_partition_used("/dev/sda").unwrap();
    let fp = get_first_partition("/dev/sda").unwrap();
    println!("Partitions Used: {}\n",cnt);
    println!("First Partition: {}\n",fp);
    let extrapart = get_extra_partitions("/dev/sda");
    println!("Extra Partitions = {}\n",extrapart.len());
    if extrapart.len() > 0 {
       for p in extrapart.iter() {
          println!("Idx: {} Name: {}\n",p.idx, p.name);
          }
       }
   erase_disk();
   add_extra_partitions("/dev/sda",extrapart);
   println!("**** Active parts\n");
   read_part();
   println!("**** All parts\n");
   read_all_part();
} 

fn erase_disk() {


    println!("Erasing Disk\n");
    let _status = Command::new("sfdisk").arg("--delete").arg("/dev/sda").status().expect("process failed to execute");
    println!();
    thread::sleep(time::Duration::from_secs(10));
    println!("Disk Erased\n");
}

fn read_part() {
   let mut f = std::fs::File::open("/dev/sda")
    .expect("could not open disk");
   let gpt = gptman::GPT::find_from(&mut f)
      .expect("could not find GPT");

   println!("Disk GUID: {:?}", gpt.header.disk_guid);

   for (i, p) in gpt.iter() {
     if p.is_used() {
        println!("Partition #{}: name = {} type = {:?}, size = {} bytes, starting lba = {}",
            i,
            p.partition_name.to_string(),
            p.partition_type_guid,
            p.size().unwrap() * gpt.sector_size,
            p.starting_lba);
            }
    }
  }
  
fn read_all_part() {
   let mut f = std::fs::File::open("/dev/sda")
    .expect("could not open disk");
   let gpt = gptman::GPT::find_from(&mut f)
      .expect("could not find GPT");

   println!("Disk GUID: {:?}", gpt.header.disk_guid);

   for (i, p) in gpt.iter() {
        println!("Partition #{}: name = {} type = {:?}, size = {} bytes, starting lba = {}",
            i,
            p.partition_name.to_string(),
            p.partition_type_guid,
            p.size().unwrap() * gpt.sector_size,
            p.starting_lba);
  }
}

fn get_partition_used(disk: &str) -> Result<u32,std::io::Error>  {
    let f = std::fs::File::open(disk.to_string());
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
        };
    let gpt = gptman::GPT::find_from(&mut f)
      .expect("GPT Partitions not found");
   let mut cnt = 0;
   for (_i, p) in gpt.iter() {
     if p.is_used() {
        cnt = cnt + 1;
        }
     }
   return Ok(cnt);
}
   
fn get_first_partition(disk: &str) -> Result<u32,std::io::Error> {
    let f = std::fs::File::open(disk.to_string());
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
        };
    let gpt = gptman::GPT::find_from(&mut f)
      .expect("GPT Partitions not found");
    for (i, p) in gpt.iter() {
     if p.is_used() {
        return Ok(i+1);
        }
     }
   return Ok(0);
}


pub struct GptPartition {
    pub idx: u32,
    pub partition_type: [u8; 16],
    pub guid: [u8; 16],
    pub start_lba: u64,
    pub end_lba: u64,
    pub attributes: u64,
    pub name: String,
}

fn get_extra_partitions(disk:&str) -> Vec<GptPartition> {
    let mut f = std::fs::File::open(disk.to_string())
      .expect("Cannot open disk");
    let gpt = gptman::GPT::find_from(&mut f)
      .expect("GPT Partitions not found");
    let mut result: Vec<GptPartition> = Vec::new();
    let mut uidx = 0;
    for (_i, p) in gpt.iter() {
        if p.is_used() {
           if uidx > 3 {
              result.push(GptPartition {
                   idx: uidx + 1,
                   partition_type: p.partition_type_guid,
                   guid: p.unique_parition_guid,
                   start_lba: p.starting_lba,
                   end_lba:   p.ending_lba,
                   //attributes: p.attribute_bits,
                   attributes: 0,
                   name:       p.partition_name.to_string(),
                   } );
              }
           uidx = uidx + 1;
           }
        }
   drop ( f );
   return result;
}

fn get_gpt_partitions(disk:&str) -> Vec<GptPartition> {
    let mut f = std::fs::File::open(disk.to_string())
      .expect("Cannot open disk");
    let gpt = gptman::GPT::find_from(&mut f)
      .expect("GPT Partitions not found");
    let mut result: Vec<GptPartition> = Vec::new();
    let mut uidx = 0;
    for (_i, p) in gpt.iter() {
        if p.is_used() {
              result.push(GptPartition {
                   idx: uidx,
                   partition_type: p.partition_type_guid,
                   guid: p.unique_parition_guid,
                   start_lba: p.starting_lba,
                   end_lba:   p.ending_lba,
                   attributes: p.attribute_bits,
                   name:       p.partition_name.to_string(),
                   } );
              }
           uidx = uidx + 1;
        }
   drop ( f );
   return result;
}

fn add_extra_partitions(disk:&str,extra_parts:Vec<GptPartition>) -> Vec<GptPartition>  {
    let mut f = std::fs::File::open(disk.to_string())
      .expect("Cannot open disk");
    let mut gpt = gptman::GPT::find_from(&mut f)
      .expect("GPT Partitions not found");
    for p in extra_parts.iter() {
        println!("Adding {} into slot {}\n",p.name,p.idx);
        println!("Start: {}\n",p.start_lba);
        println!("End:   {}\n",p.end_lba);
        //gpt[p.idx+1] = gptman::GPTPartitionEntry {
        gpt[5] = gptman::GPTPartitionEntry {
                    starting_lba:  p.start_lba,
                    ending_lba:    p.end_lba,
                    attribute_bits: p.attributes,
                    partition_name: p.name[..].into(),
                    partition_type_guid: [0xff; 16],
                    unique_parition_guid: [0xff; 16],
                    //partition_type_guid: p.partition_type,
                    //unique_parition_guid: p.guid,
                    };
        }
   drop (f) ;
   let result = get_gpt_partitions(disk);
   return result;
}

