use std::str::FromStr;
//const FP_FIELDS_1: &str = "svd/test/fields_1.svd";
//const FP_NESTED_1: &str = "svd/test/nested_1.svd";
//const FP_STRIP_DEVICE_1: &str = "svd/test/strip_device_1.svd";
//const FP_CPU_1: &str = "svd/test/cpu_1.svd";
//const FP_PERIPHERAL_1: &str = "svd/test/peripheral_1.svd";
//const FP_H743: &str = "svd/STM32H7_SVD/STM32H7_svd_V1.9/STM32H743.svd";

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

struct AddressBlock {
    offset: u32,
    size: u32,
    usage: String,
}
struct Interrupt {
    name: String,
    description: String,
    value: u32,
}
struct RegisterField {
    name: String,
    description: String,
    bit_offset: u32,
    bit_width: u32,
    access: AccessType,
}
enum AccessType {
    ReadWrite,
    ReadOnly,
    WriteOnly,
}
struct Register {
    name: String,
    display_name: String,
    description: String,
    address_offset: u32,
    size: u32,
    access: AccessType,
    reset_value: u32,
    fields: Vec<RegisterField>,
}
struct Peripheral {
    name: String,
    description: String,
    group_name: String,
    base_address: u32,
    address_block: AddressBlock,
    interrupt: Vec<Interrupt>,
    registers: Vec<Register>,
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
fn get_peripherals(svd: &str) -> Vec<String> {
    let svd = svd.replace("<peripherals>","")
        .replace("</peripherals>","");
    let mut split: Vec<String> = svd.split("<peripheral>")
                                        .map(|s| s.replace("<peripherals>","")
                                                  .replace("</peripherals>","")
                                                  .replace("</peripheral>",""))
                                        .map(|s| s.trim().to_string())
                                        .collect();
    split
}

fn parse_peripheral(peripheral: &str) {
    println!("Pretending to parse\n{}",peripheral);
    /*
    Peripheral {
        name:
        description:
        group_name:
        base_address:
        address_block:
        interrupt:
        registers:
    }
    */
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn leftover_from_initial_dev_test() {
        const FP_STRIP_DEVICE_1: &str = "svd/test/strip_device_1.svd";
        const FP_CPU_1: &str = "svd/test/cpu_1.svd";
        let svd_3 = fs::read_to_string(FP_STRIP_DEVICE_1).unwrap();
        let svd_3 = strip_device(&svd_3);
        println!("{}", svd_3);

        let svd_cpu = fs::read_to_string(FP_CPU_1).unwrap();
        let svd_cpu = strip_device(&svd_cpu);

        let cpu = get_cpu(svd_cpu);
        println!("{:?}", cpu);
    }
    #[test]
    fn read_peripheral_to_string() {
        const FP_PERIPHERAL_1: &str = "svd/test/peripheral_1.svd";
        const FP_PERIPHERAL_1_FIRST_PERIPHERAL: &str = "svd/test/peripheral_1_first_peripheral.svd";
        const FP_PERIPHERAL_1_SECOND_PERIPHERAL: &str = "svd/test/peripheral_1_second_peripheral.svd";
        let svd_peripheral_1 = fs::read_to_string(FP_PERIPHERAL_1).unwrap();
        let peripherals = get_peripherals(&svd_peripheral_1);
        assert_eq!(peripherals[0],fs::read_to_string(FP_PERIPHERAL_1_FIRST_PERIPHERAL).unwrap());
        assert_eq!(peripherals[1],fs::read_to_string(FP_PERIPHERAL_1_SECOND_PERIPHERAL).unwrap());

    }
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
