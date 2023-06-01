# Plonky 2 Multiplexer
Dual mux for 32 byte hashes. A piece of a sha256 merkle proof verifier. Inspired by [this circuit](https://github.com/tornadocash/tornado-core/blob/master/circuits/merkleTree.circom#L18)

# Purpose
1. Signature aggregation for tendermint headers, to speed up and streamline Celestia Light node sync, so it can more-easily fit into a web browser or smart phone someday.
2. Witness compression for Cosmos-SDK IAVL tree state proofs, for usage in Rollkit.
