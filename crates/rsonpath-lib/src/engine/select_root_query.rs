//! Special case handlers for the empty query $.
//!
//! The main engine is built with a path automaton in mind, and for simplicity
//! we assume the root opening was already read. This makes it incompatible with
//! an empty query. Instead of rewriting the engine we provide fast-path implementations
//! here.
use crate::{
    engine::{error::EngineError, Input},
    input::{error::InputErrorConvertible, InputBlockIterator},
    is_json_whitespace,
    result::{empty::EmptyRecorder, Match, MatchCount, MatchIndex, MatchSpan, Sink},
    BLOCK_SIZE,
};
use crate::result::InputRecorder;

/// Count for an empty query &ndash; determine if the root exists.
pub(super) fn count<'i, 'r, I, R, const N: usize>(input: &I) -> Result<MatchCount, EngineError>
where
    I: Input<'i, 'r, R, N>,
    R: InputRecorder<I::Block> + 'r
{
    // Assuming a correct JSON, there is either one root if any non-whitespace character
    // occurs in the document, or the document is empty.
    if input.seek_non_whitespace_forward(0).e()?.is_some() {
        Ok(1)
    } else {
        Ok(0)
    }
}

/// Index for an empty query &ndash; determine the first index of the root.
pub(super) fn index<'i, 'r, I, R, S, const N: usize>(input: &I, sink: &mut S) -> Result<(), EngineError>
where
    I: Input<'i, 'r, R, N>,
    R: InputRecorder<I::Block> + 'r,
    S: Sink<MatchIndex>,
{
    // Assuming a correct JSON, the root starts at the first non-whitespace character, if any.
    if let Some((first_idx, _)) = input.seek_non_whitespace_forward(0).e()? {
        sink.add_match(first_idx - input.leading_padding_len())
            .map_err(|err| EngineError::SinkError(Box::new(err)))?;
    }

    Ok(())
}

/// Approximate span for an empty query &ndash; determine the first index and the length of the root.
pub(super) fn approx_span<'i, 'r, I, R, S, const N: usize>(input: &I, sink: &mut S) -> Result<(), EngineError>
where
    I: Input<'i, 'r, R, N>,
    R: InputRecorder<I::Block> + 'r,
    S: Sink<MatchSpan>,
{
    // The root spans the entire document, by definition, with the exception of whitespace.
    // We need to find the start index exactly, and then can return the length of the rest as the approximate
    // length of the root.
    //
    // Some input know their lengths: bytes already in memory, file mmaps, etc.
    // A BufferedInput over an arbitrary Read stream cannot know its length, so we actually
    // need to iterate until the end and count the bytes.
    if let Some((first_idx, _)) = input.seek_non_whitespace_forward(0).e()? {
        let end_idx = match input.len_hint() {
            Some(end_idx) => end_idx, // Known length, just take it.
            None => {
                // Unknown length, iterate and count.
                let mut iter = input.iter_blocks(&EmptyRecorder);
                let mut end_idx = 0;

                while (iter.next().e()?).is_some() {
                    end_idx += BLOCK_SIZE;
                }

                end_idx
            }
        };

        sink.add_match(MatchSpan::from_indices(
            first_idx - input.leading_padding_len(),
            end_idx - input.leading_padding_len(),
        ))
        .map_err(|err| EngineError::SinkError(Box::new(err)))?;
    }

    Ok(())
}

/// Match for an empty query &ndash; copy the entire document, trimming whitespace.
pub(super) fn match_<'i, 'r, I, R, S, const N: usize>(input: &I, sink: &mut S) -> Result<(), EngineError>
where
    I: Input<'i, 'r, R, N>,
    R: InputRecorder<I::Block> + 'r,
    S: Sink<Match>,
{
    // For a full match we need to copy the entire input starting from first non-whitespace,
    // and then trim the whitespace from the end. This might be slow if the document is excessively
    // padded with whitespace at start and/or end, but that's a pathological case.
    let mut iter = input.iter_blocks(&EmptyRecorder);
    let mut res: Vec<u8> = vec![];
    let mut first_significant_idx = None;
    let mut offset = 0;

    while let Some(block) = iter.next().e()? {
        if first_significant_idx.is_none() {
            // Start of the root not found yet, look for it.
            first_significant_idx = block.iter().position(|&x| !is_json_whitespace(x));

            if let Some(first_idx) = first_significant_idx {
                // Start of the root found in this block, copy the relevant part.
                res.extend(&block[first_idx..]);
            } else {
                offset += block.len();
            }
        } else {
            // Start of the root was already found, now we are copying everything.
            res.extend(&*block);
        }
    }

    if let Some(start) = first_significant_idx {
        // Trim whitespace if we have a result.
        while !res.is_empty() && is_json_whitespace(res[res.len() - 1]) {
            res.pop();
        }

        let actual_start = start + offset - input.leading_padding_len();
        sink.add_match(Match::from_start_and_bytes(actual_start, res))
            .map_err(|err| EngineError::SinkError(Box::new(err)))?;
    }

    Ok(())
}
