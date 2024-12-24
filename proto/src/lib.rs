// It is important to maintain the same structure as in the proto.
pub mod dev {
    pub mod vsx {
        pub mod svctl {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/dev.vsx.svctl.v1.rs"));
            }
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
