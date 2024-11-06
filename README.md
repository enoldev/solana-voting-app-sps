# Solana Substreams-powered Subgraph (Solana Voting Application)

This is a [Substreams-powered Subgraph](https://thegraph.com/docs/en/sps/introduction/) that retrieves data from the [Solana Foundation's Example Voting Application](https://github.com/solana-developers/developer-bootcamp-2024/tree/main/project-2-voting).

## Get Started

The Substreams has been auto-generated (using the `substreams init` CLI) from the JSON Anchor IDL of the Voting Application, which is deployed in Solana Devnet with Program ID `AaxYiadTQSPFswtmEpjZ15vQnjzkryHhgnJz2GjxxbeV`.

### Run the Substreams

Everything is already packeged in the `my-project-v0.1.0.spkg` file, which you can directly run using the Substreams CLI.

```bash
substreams gui ./my-project-v0.1.0.spkg map_program_data --start-block=337067528 --stop-block=+3000 -e devnet.sol.streamingfast.io:443
```

### Run the Substreams-powered Subgraph

A Substreams-powered Subgraph (SpS) has been generated from the `.spkg` file by using the `substreams codegen subgraph ./my-project-v0.1.0.spkg` command, and the `mappings.ts` file has been updated to create the corresponding entities of the subgraph.

You can deploy the SpS using the standard `graph deploy` command, either on The Graph Studio or a local Graph Node.