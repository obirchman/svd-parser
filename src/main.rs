use std::{fs, str::FromStr};
const FP_1: &str = "svd/test_1.svd";
const FP_2: &str = "svd/test_2.svd";
const FP_3: &str = "svd/test_3.svd";
const FP_CPU: &str = "svd/test_cpu.svd";
const FP_H743: &str = "svd/STM32H7_SVD/STM32H7_svd_V1.9/STM32H743.svd";

#[derive(Debug)]
enum CPUname{
    CM0,
    CM0plus,
    CM4,
    CM7,
    Unknown(String),
}
impl FromStr for CPUname {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CM0" => Ok(CPUname::CM0),
            "CM0plus" => Ok(CPUname::CM0plus),
            "CM4" => Ok(CPUname::CM4),
            "CM7" => Ok(CPUname::CM7),
            _ => Ok(CPUname::Unknown(s.to_string())),
        }
    }
}
#[derive(Debug)]
enum Endian {
    Little,
    Big,
    Unknown(String),
}
impl FromStr for Endian {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "little" => Ok(Endian::Little),
            "big" => Ok(Endian::Big),
            _ => Ok(Endian::Unknown(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct CPU {
    name: CPUname,
    revision: String,
    endian: Endian,
    mpu_present: bool,
    fpu_present: bool,
    nvic_prio_bits: u8,
    vendor_systick_config: bool,
}
fn get_field<'a>(tag: &'a str,svd: &'a str) -> &'a str{
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);
    svd.split_once(&start_tag).unwrap()
        .1.split_once(&end_tag).unwrap()
        .0
}

fn strip_device(svd: &str) -> &str {
    svd.split_once("<device").unwrap()
    .1.split_once(">").unwrap()
    .1.rsplit_once("</device>").unwrap()
    .0
}

fn get_cpu(svd: &str) -> CPU {
    let cpu_fields = svd.split_once("<cpu>").unwrap()
        .1.split_once("</cpu>").unwrap()
        .0;
    CPU {
        name: CPUname::from_str(get_field("name", cpu_fields)).unwrap(),
        revision: get_field("revision", cpu_fields).to_string(),
        endian: Endian::from_str(get_field("endian", cpu_fields)).unwrap(),
        mpu_present: get_field("mpuPresent", cpu_fields).parse().unwrap(),
        fpu_present: get_field("fpuPresent", cpu_fields).parse().unwrap(),
        nvic_prio_bits: get_field("nvicPrioBits", cpu_fields).parse().unwrap(),
        vendor_systick_config: get_field("vendorSystickConfig", cpu_fields).parse().unwrap(),
    }
}

fn main() {
    let svd_3 = fs::read_to_string(FP_3).unwrap();
    let svd_3 = strip_device(&svd_3);
    println!("{}", svd_3);
    
    let svd_cpu = fs::read_to_string(FP_CPU).unwrap();
    let svd_cpu = strip_device(&svd_cpu);
    let cpu = get_cpu(svd_cpu);
    println!("{:?}", cpu);

}

fn get_peripherals() {

}

fn parse_peripheral() {

}

fn parse_address_block() {

}

fn parse_interrupt() {

}

fn get_registers() {

}

fn parse_register() {

}

fn get_fields() {

}

fn parse_field() {

}
