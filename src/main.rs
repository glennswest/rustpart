
fn main() {

    println!("Partition Test\n");
    read_part();
    let cnt = get_partition_used("/dev/sda").unwrap();
    let fp = get_first_partition("/dev/sda").unwrap();
    println!("Partitions Used: {}\n",cnt);
    println!("First Partition: {}\n",fp);
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
