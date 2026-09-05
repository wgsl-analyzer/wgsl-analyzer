//! The byte-copy helper shared by both pipes.
//!
//! Compiled on every target, unlike the rest of the crate, so the fiddliest
//! logic in the bridge is reachable from `cargo test` on the host.

#![cfg_attr(
    not(target_os = "emscripten"),
    allow(
        dead_code,
        reason = "only the emscripten build calls this, but the tests below still cover it"
    )
)]

use std::collections::VecDeque;

/// Move as many bytes as fit from the front of `queue` into `dst`, removing
/// them. Returns the number moved, which is `min(queue.len(), dst.len())`.
///
/// A `VecDeque` holds its contents as up to two slices, so this copies both
/// halves instead of stepping a byte at a time. LSP responses run to tens of
/// kilobytes and every one of them passes through here.
pub(crate) fn drain_into(
    queue: &mut VecDeque<u8>,
    dst: &mut [u8],
) -> usize {
    let count = dst.len().min(queue.len());
    let (front, back) = queue.as_slices();
    let from_front = count.min(front.len());

    dst[..from_front].copy_from_slice(&front[..from_front]);
    dst[from_front..count].copy_from_slice(&back[..count - from_front]);

    queue.drain(..count);
    count
}

#[cfg(test)]
mod tests {
    use super::drain_into;
    use std::collections::VecDeque;

    /// A deque holding `0..8` whose contents wrap around the end of its ring
    /// buffer, so `as_slices` returns two non-empty halves.
    fn wrapped() -> VecDeque<u8> {
        let mut queue = VecDeque::with_capacity(8);
        for byte in 4..8 {
            queue.push_back(byte);
        }
        for byte in (0..4).rev() {
            queue.push_front(byte);
        }
        assert!(
            !queue.as_slices().1.is_empty(),
            "this test is pointless unless the deque is split"
        );
        queue
    }

    #[test]
    fn empty_queue_moves_nothing() {
        let mut queue = VecDeque::new();
        let mut destination = [0xAA; 4];
        assert_eq!(drain_into(&mut queue, &mut destination), 0);
        assert_eq!(destination, [0xAA; 4]);
    }

    #[test]
    fn empty_destination_moves_nothing() {
        let mut queue = wrapped();
        assert_eq!(drain_into(&mut queue, &mut []), 0);
        assert_eq!(queue.len(), 8);
    }

    #[test]
    fn short_destination_takes_a_prefix() {
        let mut queue = wrapped();
        let mut destination = [0; 3];
        assert_eq!(drain_into(&mut queue, &mut destination), 3);
        assert_eq!(destination, [0, 1, 2]);
        assert_eq!(
            queue.iter().copied().collect::<Vec<_>>(),
            vec![3, 4, 5, 6, 7]
        );
    }

    #[test]
    fn destination_stopping_inside_the_front_half() {
        let mut queue = wrapped();
        let mut destination = [0; 4];
        assert_eq!(drain_into(&mut queue, &mut destination), 4);
        assert_eq!(destination, [0, 1, 2, 3]);
        assert_eq!(queue.iter().copied().collect::<Vec<_>>(), vec![4, 5, 6, 7]);
    }

    #[test]
    fn destination_spanning_the_split() {
        let mut queue = wrapped();
        let mut destination = [0; 6];
        assert_eq!(drain_into(&mut queue, &mut destination), 6);
        assert_eq!(destination, [0, 1, 2, 3, 4, 5]);
        assert_eq!(queue.iter().copied().collect::<Vec<_>>(), vec![6, 7]);
    }

    #[test]
    fn exact_fit_drains_the_queue() {
        let mut queue = wrapped();
        let mut destination = [0; 8];
        assert_eq!(drain_into(&mut queue, &mut destination), 8);
        assert_eq!(destination, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert!(queue.is_empty());
    }

    #[test]
    fn oversized_destination_takes_everything_and_no_more() {
        let mut queue = wrapped();
        let mut destination = [0xAA; 12];
        assert_eq!(drain_into(&mut queue, &mut destination), 8);
        assert_eq!(destination[..8], [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(destination[8..], [0xAA; 4]);
        assert!(queue.is_empty());
    }

    #[test]
    fn repeated_calls_preserve_order() {
        let mut queue = wrapped();
        let mut seen = Vec::new();
        let mut destination = [0; 3];
        loop {
            let moved = drain_into(&mut queue, &mut destination);
            if moved == 0 {
                break;
            }
            seen.extend_from_slice(&destination[..moved]);
        }
        assert_eq!(seen, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }
}
