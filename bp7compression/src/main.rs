use bp7::{bundle, canonical, crc, dtntime, eid, primary, Bundle, ByteBuffer};
use bp7compression::*;
use lipsum::lipsum_words;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;

fn print_sorted_by_value(data: &HashMap<&'static str, usize>) {
    let mut str_vec: Vec<(&&'static str, &usize)> = data.iter().collect();
    str_vec.sort_by(|a, b| b.1.cmp(a.1));
    for a in str_vec.iter() {
        println!(
            "{} : {} : {:.2}",
            a.0,
            a.1,
            *a.1 as f32 / data["raw"] as f32
        );
    }
}
fn print_sorted_by_name(data: &HashMap<&'static str, usize>) {
    let mut str_vec: Vec<(&&'static str, &usize)> = data.iter().collect();
    str_vec.sort_by(|a, b| b.0.cmp(a.0));
    for a in str_vec.iter() {
        println!(
            "{} : {} : {:.2}",
            a.0,
            a.1,
            *a.1 as f32 / data["raw"] as f32
        );
    }
}
fn compress_eval_bundle(bundle: Vec<u8>, id_txt: &str) {
    //let mut str_compression: HashMap<&'static str, usize> = HashMap::new();
    let mut bin_compression: HashMap<&'static str, usize> = HashMap::new();
    //let bundle_str = "9f8907000182016b6e6f6465332f696e626f7882016b6e6f6465332f696e626f7882016b6e6f6465332f696e626f78821a247966ba001ad693a4004225b686010000014341424342237186080100010042dbccff";
    //let bundle_str = "9f8907000182016b6e6f6465332f696e626f7882016b6e6f6465322f696e626f7882016b6e6f6465322f696e626f78821a251f8061001ad693a400422a6886010000014341424344454647484950515242237186080100010042dbccff";
    //let bundle_str = "9f8907000182016b6e6f6465332f696e626f7882016b6e6f6465332f696e626f7882016b6e6f6465332f696e626f78821a247966ba001ad693a4004225b686010000014341424342237186080100010042dbccff9f8907000182016b6e6f6465332f696e626f7882016b6e6f6465322f696e626f7882016b6e6f6465322f696e626f78821a251f8061001ad693a400422a6886010000014341424344454647484950515242237186080100010042dbccff";
    //let bundle_str = "9f8907000182016b6e6f6465332f696e626f7882016b6e6f6465332f696e626f7882016b6e6f6465332f696e626f78821a247966ba001ad693a4004225b686010000014341424342237186080100010042dbccff9f8907000182016b6e6f6465332f696e626f7882016b6e6f6465332f696e626f7882016b6e6f6465332f697e626f78821a247966ba001ad693a8004225b686010000014341424342237186080100010042dbccff";

    //str_compression.insert("raw", bundle_str.len());
    bin_compression.insert("raw", bundle.len());

    /*str_compression.insert(
        "libflate_gzip",
        libflate_gzip_compress(bundle_str.as_bytes()).len(),
    );*/
    bin_compression.insert("libflate_gzip", libflate_gzip_compress(&bundle).len());
    /*str_compression.insert(
        "libflate_deflate",
        libflate_deflate_compress(bundle_str.as_bytes()).len(),
    );*/
    bin_compression.insert("libflate_deflate", libflate_deflate_compress(&bundle).len());
    /*str_compression.insert(
        "libflate_zlib",
        libflate_zlib_compress(bundle_str.as_bytes()).len(),
    );*/
    bin_compression.insert("libflate_zlib", libflate_zlib_compress(&bundle).len());

    //str_compression.insert("snap", snap_compress(bundle_str.as_bytes()).len());
    bin_compression.insert("snap", snap_compress(&bundle).len());

    //str_compression.insert("bz2", bz2_compress(bundle_str.as_bytes()).len());
    bin_compression.insert("bz2", bz2_compress(&bundle).len());
    //str_compression.insert("brotli", brotli_compress(bundle_str.as_bytes()).len());
    bin_compression.insert("brotli", brotli_compress(&bundle).len());
    //str_compression.insert("miniz", miniz_compress(bundle_str.as_bytes()).len());
    bin_compression.insert("miniz", miniz_compress(&bundle).len());
    //str_compression.insert("xz2", xz2_compress(bundle_str.as_bytes()).len());
    bin_compression.insert("xz2", xz2_compress(&bundle).len());

    //str_compression.insert("smaz", smaz_compress(bundle_str.as_bytes()).len());
    bin_compression.insert("smaz", smaz_compress(&bundle).len());
    //dbg!(&str_compression);
    //dbg!(&bin_compression);

    /*println!("# string compressions");
    print_sorted_by_name(&str_compression);
    println!();*/
    println!("# {} compressions by name", id_txt);
    print_sorted_by_name(&bin_compression);
    println!();
    println!();
    /*println!("# string compressions");
    print_sorted_by_value(&str_compression);
    println!();*/
    //println!("# {} compressions by value", id_txt);
    //print_sorted_by_value(&bin_compression);
}

fn get_minimal_bundle(crc_type: crc::CRCType, payload: &[u8]) -> Bundle {
    let dst = eid::EndpointID::with_dtn("node2/inbox");
    let src = eid::EndpointID::with_dtn("node1/123456");
    //let dst = eid::EndpointID::with_ipn(eid::IpnAddress(1, 2));
    //let src = eid::EndpointID::with_ipn(eid::IpnAddress(2, 3));
    let now = dtntime::CreationTimestamp::with_time_and_seq(dtntime::dtn_time_now(), 0);
    //let now = dtntime::CreationTimestamp::with_time_and_seq(dtntime::DTN_TIME_EPOCH, 0);

    //let pblock = primary::new_primary_block("dtn:node2/inbox".to_string(), "dtn:node1/123456".to_string(), now, 60 * 60 * 1_000_000);
    let pblock = primary::PrimaryBlockBuilder::default()
        .destination(dst)
        .source(src.clone())
        .report_to(bp7::eid::EndpointID::with_dtn_none())
        .creation_timestamp(now)
        .lifetime(60 * 60 * 1_000_000)
        .build()
        .unwrap();
    let cblocks = vec![canonical::new_payload_block(0, payload.to_vec())];
    let mut b = bundle::Bundle::new(pblock, cblocks);
    b.set_crc(crc_type);
    b.validation_errors();
    b
}

fn get_big_bundle(crc_type: crc::CRCType, payload: &[u8]) -> Bundle {
    let dst = eid::EndpointID::with_dtn("node2/inbox");
    let src = eid::EndpointID::with_dtn("node1/123456");
    //let dst = eid::EndpointID::with_ipn(eid::IpnAddress(1, 2));
    //let src = eid::EndpointID::with_ipn(eid::IpnAddress(2, 3));
    let now = dtntime::CreationTimestamp::with_time_and_seq(dtntime::dtn_time_now(), 0);
    //let now = dtntime::CreationTimestamp::with_time_and_seq(dtntime::DTN_TIME_EPOCH, 0);

    //let pblock = primary::new_primary_block("dtn:node2/inbox".to_string(), "dtn:node1/123456".to_string(), now, 60 * 60 * 1_000_000);
    let pblock = primary::PrimaryBlockBuilder::default()
        .destination(dst)
        .source(src.clone())
        .report_to(src)
        .creation_timestamp(now)
        .lifetime(60 * 60 * 1_000_000)
        .build()
        .unwrap();
    let cblocks = vec![
        canonical::new_payload_block(0, payload.to_vec()),
        canonical::new_bundle_age_block(
            2, // block number
            0, // flags
            0, // time elapsed
        ),
        canonical::new_hop_count_block(
            3, // block number
            0, 16,
        ),
    ];
    let mut b = bundle::Bundle::new(pblock, cblocks);
    b.set_crc(crc_type);
    b.validation_errors();
    b
}
fn main() {
    println!("Pure short bundle\n");
    //let bundle_str = "9f8907000182016b6e6f6465332f696e626f7882016b6e6f6465322f696e626f7882016b6e6f6465322f696e626f78821a251f8061001ad693a400422a6886010000014341424344454647484950515242237186080100010042dbccff";
    //let bundle = unhexify(bundle_str).unwrap();
    let bundle = get_minimal_bundle(bp7::crc::CRC_32, b"ABCDEFG").to_cbor();
    compress_eval_bundle(bundle, "min bundle");

    println!("\n\n\nPure large bundle\n");
    //let bundle_str = "9f8907000182016b6e6f6465332f696e626f7882016b6e6f6465322f696e626f7882016b6e6f6465322f696e626f78821a251f8061001ad693a400422a6886010000014341424344454647484950515242237186080100010042dbccff";
    //let bundle = unhexify(bundle_str).unwrap();
    let bundle = get_big_bundle(bp7::crc::CRC_32, b"ABCDEFG").to_cbor();
    compress_eval_bundle(bundle, "max bundle");

    println!("\n\n\nPure short lipsum text\n");
    let text_str = lipsum_words(15);
    compress_eval_bundle(text_str.as_bytes().to_vec(), "short text");

    println!("\n\n\nPure long lipsum text\n");
    let text_str = lipsum_words(2000);
    compress_eval_bundle(text_str.as_bytes().to_vec(), "long text");

    println!("\n\n\nShort text bundle\n");
    let text_str = lipsum_words(15);
    let bundle = get_minimal_bundle(bp7::crc::CRC_32, text_str.as_bytes()).to_cbor();
    compress_eval_bundle(bundle, "text bundle");

    println!("\n\n\nPNG bundle\n");
    let contents = fs::read("data/enwiki.png").expect("Something went wrong reading the file");
    let bundle = get_minimal_bundle(bp7::crc::CRC_16, &contents).to_cbor();
    compress_eval_bundle(bundle, "png bundle");

    // https://www.heagmobibus.de/sites/default/files/media/Airliner_Faltblatt_2019.pdf
    println!("\n\n\nPDF bundle\n");
    let contents = fs::read("data/Airliner_Faltblatt_2019.pdf")
        .expect("Something went wrong reading the file");
    let bundle = get_minimal_bundle(bp7::crc::CRC_16, &contents).to_cbor();
    compress_eval_bundle(bundle, "pdf bundle");
}
