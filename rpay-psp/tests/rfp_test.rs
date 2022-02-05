use rpay_core::rfp::traits::Rfp;

struct DumbRfp;

impl DumbRfp {
    fn new() -> Self {
        DumbRfp {}
    }
}

impl Rfp for DumbRfp {
    fn create() {
        todo!()
    }

    fn delete() {
        todo!()
    }

    fn publish() {
        todo!()
    }
}

#[test]
fn run_rfp_tests() {
    let rfp = DumbRfp::new();

    assert_eq!(Rfp, );
}
