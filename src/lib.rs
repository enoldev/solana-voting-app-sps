mod idl;
mod pb;

use anchor_lang::AnchorDeserialize;
use anchor_lang::Discriminator;
use base64::prelude::*;
use pb::substreams::v1::program::Candidate;
use pb::substreams::v1::program::Data;
use pb::substreams::v1::program::InitializeCandidate;
use pb::substreams::v1::program::InitializePoll;
use pb::substreams::v1::program::Poll;
use pb::substreams::v1::program::Vote;

use sologger_log_context::programs_selector::ProgramsSelector;
use sologger_log_context::sologger_log_context::LogContext;
use substreams::log::println;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

const PROGRAM_ID: &str = "AaxYiadTQSPFswtmEpjZ15vQnjzkryHhgnJz2GjxxbeV";

#[substreams::handlers::map]
fn map_program_data(blk: Block) -> Data {
    let mut initialize_candidate_list: Vec<InitializeCandidate> = Vec::new();
    let mut initialize_poll_list: Vec<InitializePoll> = Vec::new();
    let mut vote_list: Vec<Vote> = Vec::new();

    blk.transactions().for_each(|transaction| {
        let meta_wrapped = &transaction.meta;
        let meta = meta_wrapped.as_ref().unwrap();

        // ------------- INSTRUCTIONS -------------
        transaction
            .walk_instructions()
            .into_iter()
            .filter(|inst| inst.program_id().to_string() == PROGRAM_ID)
            .for_each(|inst| {
                let slice_u8: &[u8] = &inst.data()[..];
                if slice_u8[0..8]
                    == idl::idl::program::client::args::InitializeCandidate::DISCRIMINATOR
                {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::InitializeCandidate::deserialize(
                            &mut &slice_u8[8..],
                        )
                    {
                        initialize_candidate_list.push(InitializeCandidate {
                            candidate_name: instruction.candidate_name,
                            poll_id: instruction._poll_id,
                        });
                    }
                }
                if slice_u8[0..8] == idl::idl::program::client::args::InitializePoll::DISCRIMINATOR
                {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::InitializePoll::deserialize(
                            &mut &slice_u8[8..],
                        )
                    {
                        initialize_poll_list.push(InitializePoll {
                            poll_id: instruction.poll_id,
                            description: instruction.description,
                            poll_start: instruction.poll_start,
                            poll_end: instruction.poll_end,
                        });
                    }
                }
                if slice_u8[0..8] == idl::idl::program::client::args::Vote::DISCRIMINATOR {
                    if let Ok(instruction) =
                        idl::idl::program::client::args::Vote::deserialize(&mut &slice_u8[8..])
                    {
                        vote_list.push(Vote {
                            candidate_name: instruction._candidate_name,
                            poll_id: instruction._poll_id,
                        });
                    }
                }
            });
    });

    Data {
        initialize_candidate_list,
        initialize_poll_list,
        vote_list,
    }
}
