use utils::benchmark_v2;
use zkm_script::{benchmark_reth, init_logger};

fn main() {
    init_logger();

    benchmark_v2(
        benchmark_reth,
        &[0],
        "../benchmark_outputs/reth_ziren.csv",
        "",
    );
}
