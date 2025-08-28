use dprint_core::formatting::PrintItems;

pub enum PrintItemRequest {
    OneSpace,
}

// The motivating example for this is, that there is no obvious way to encode the following rules cleanly into "vanilla" PrintItems
// 1. There should not be a space between the name of a function and the opening parenthesis "fn main("
// 2. A block comment (/* aaa */) should be preceded and followed by a space
// 3. There should not be a space after the opening parenthesis of a function, even if the next token is a block comment
// 4. There should not be a space before the closing parenthesis of a function, even if the preceding token is a block comment
//
// Example formattings: fn main /*aaa*/ (/*bbb*/ param: u32, param2: u32 /*ccc*/)
//
// Considered alternatives:
// * Track if the last pushed item is a space, and branch on that everytime you would add a space
//   * Cons: Very verbose, imperative and brittle ("forget to update the last pushed item"), cannot deal with rule 4 properly.
// * "Cleverly" structure code and where to put spaces, so that these cases are implicitly dealt with
//   * Cons: "Clever" code that doesn't explicitly state intent, and thus is brittle, new requirements might require big restructurings
// * Re-parse the AST into a formatting-ast which tracks comments etc.
//   * Cons: Feels like this just postpones the problem, a lot of boilerplate "if the next printed item is a comment, add a space"
//
// Chosen solution:
// * Feels like it can most clearly encode the intent behind statements like
//   "add a comma, unless its followed by ')'" or "there should be a single space after 'fn' and before the name"
// * In the formatting code we don't actually care about "what exactly the next or previous token is", instead
//   we wan't to communicate that we may want separation to adjacent text.
/// A wrapper for PrintItems which adds the ability to do "item-requests"
///
/// In a lot of places the intent is to have code of a particular shape, depending on its surroundings.
/// "Add a space, if the previous item was something that we need to separation from"
///
/// All formatting should go through this struct, which keeps track of "Requests".
/// Example:
/// * Snippet A requests that there should be a space after it `AAA|_|`
/// * Snippet B requests that there should be a space after and in front of it `|_|BBB|_|`
/// * Snippet C requests that there may never be a space in front of it `|X|CCC`
///
/// The PrintItemBuffer automatically tracks and resolves these requests, so that the outcome will be
/// `AAA BBBCCC`, where the two spaces between A and B were collapsed and the space after B was overwritten
///
/// Known downsides to this soution:
/// * Does not integrate at all with dprint's conditionals
/// * Another layer ontop of dprint's IR, which doesn't feel like it should be necessary
///
pub struct PrintItemBuffer {
    pub items: PrintItems,
    pub requests: Vec<PrintItemRequest>,
}

impl PrintItemBuffer {
    pub fn new() -> Self {
        Self {
            items: PrintItems::new(),
            requests: Vec::new(),
        }
    }
}
