

fn main() {

    let mut f = std::fs::File::open("/dev/sda")
      .expect("Cannot open disk");
    let mut gpt = gptman::GPT::find_from(&mut f)
      .expect("GPT Partitions not found");
        gpt[1] = gptman::GPTPartitionEntry {
                    partition_type_guid: [0xff; 16],
                    unique_parition_guid: [0xff; 16],
                    starting_lba:   247220224,
                    ending_lba:     249000000,
                    attribute_bits: 0,
                    partition_name: "datastore".into(),
                    };
   drop (f) ;
}


