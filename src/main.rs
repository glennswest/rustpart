extern crate serde;
extern crate serde_json;





fn main() {

    println!("Partition Test\n");
    read_part();
    let cnt = get_partition_used("/dev/sda").unwrap();
    let fp = get_first_partition("/dev/sda").unwrap();
    println!("Partitions Used: {}\n",cnt);
    println!("First Partition: {}\n",fp);
    get_partition_json("/dev/sda");
}

fn read_part() {
   let mut f = std::fs::File::open("/dev/sda")
    .expect("could not open disk");
   let gpt = gptman::GPT::find_from(&mut f)
      .expect("could not find GPT");

   println!("Disk GUID: {:?}", gpt.header.disk_guid);

   for (i, p) in gpt.iter() {
     if p.is_used() {
        println!("Partition #{}: type = {:?}, size = {} bytes, starting lba = {}",
            i,
            p.partition_type_guid,
            p.size().unwrap() * gpt.sector_size,
            p.starting_lba);
    }
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
        return Ok(i);
        }
     }
   return Ok(0);
}


pub struct GptPartition {
    pub idx: u32,
    pub partition_type: String,
    pub guid: String,
    pub start_lba: u64,
    pub end_lba: u64,
    pub fstype: String,
    pub attriubutes: u64,
    pub name: String,
}

fn get_extra_partitions(disk:&str) -> Result<Vec<GptPartition>> {
    let mut f = std::fs::File::open(disk.to_string())
      .expect("Cannot open disk");
    let gpt = gptman::GPT::find_from(&mut f)
      .expect("GPT Partitions not found");
    let mut result: Vec<GptPartition> = Vec::new();
    for (_i, p) in gpt.iter() {
        if p.is_used() {
           if uidx > 4 {
              result.push(GptPartition {
                   idx: uidx,
                   partition_type: p.partition_type_guid,
                   guid: p.unique_parition_guid,
                   start_lba: p.starting_lba,
                   end_lba:   p.ending_lba,
                   attributes: p.attribute_bits,
                   name:       String::from_utf8_lossy(&p.partition_name),
                   } );
           uidx = uidx + 1;
           }
        }


}

fn get_partition_json(disk: &str) -> String {
    let mut f = std::fs::File::open(disk.to_string())
      .expect("Cannot open disk");
    let gpt = gptman::GPT::find_from(&mut f)
      .expect("GPT Partitions not found");
    let hdr = serde_json::to_string(&gpt.header).unwrap();
    println!("{}",hdr);
    let mut uidx = 0;
    for (_i, p) in gpt.iter() {
        if p.is_used() {
           let j = serde_json::to_string(&p).unwrap();
           println!("x: {} {}", uidx,j);
           uidx = uidx + 1;
           }
        }


    return "".to_string();
}

