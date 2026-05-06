use crate::{
    engine::Engine,
    poll::{PolicyResult, Poll},
};

// go through everyone who voted in the poll
// if they are able to vote, count them toward their respective side
// if the percentage of one side is > 50% of the total vote weight, that side wins
// otherwise, inconclusive
pub fn majority(poll: &Poll, eng: &Engine) -> PolicyResult {
    PolicyResult::Accept
}

// if the weight of one side > the other, that side wins
// if the weights are equal, inconclusive
pub fn winning_vote(poll: &Poll, eng: &Engine) -> PolicyResult {
    PolicyResult::Accept
}
