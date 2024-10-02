use fairblock_proto::fairyring::keyshare::QueryVerifiableRandomnessQuery;

fn main() {
    let request = QueryVerifiableRandomnessQuery {};
    println!("{:?}", request);
}
