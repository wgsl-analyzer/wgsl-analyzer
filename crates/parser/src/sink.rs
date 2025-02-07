use super::{
	Parse,
	ParserDefinition,
	TokenKind,
	event::Event,
	lexer::Token,
	parsing::ParseError,
};
use rowan::{
	GreenNodeBuilder,
	Language,
};
use std::{
	marker::PhantomData,
	mem,
};

pub(crate) struct Sink<'t, 'input, P: ParserDefinition> {
	builder: GreenNodeBuilder<'static>,
	tokens: &'t [Token<'input, P::TokenKind>],
	cursor: usize,
	events: Vec<Event<P>>,
	errors: Vec<ParseError<P>>,
}

impl<'t, 'input, P: ParserDefinition> Sink<'t, 'input, P> {
	pub(crate) fn new(
		tokens: &'t [Token<'input, P::TokenKind>],
		events: Vec<Event<P>>,
	) -> Self {
		Self {
			builder: GreenNodeBuilder::new(),
			tokens,
			cursor: 0,
			events,
			errors: Vec::new(),
		}
	}

	pub(crate) fn finish(mut self) -> Parse<P> {
		for idx in 0..self.events.len() {
			match mem::replace(&mut self.events[idx], Event::Placeholder) {
				Event::StartNode {
					kind,
					forward_parent,
				} => {
					let mut kinds = vec![kind];

					let mut idx = idx;
					let mut forward_parent = forward_parent;

					// Walk through the forward parent of the forward parent and the forward parent
					// of that, and of that, etc. until we reach a StartNode event without a forward
					// parent.
					while let Some(fp) = forward_parent {
						idx += fp;

						forward_parent = if let Event::StartNode {
							kind,
							forward_parent,
						} = mem::replace(&mut self.events[idx], Event::Placeholder)
						{
							kinds.push(kind);
							forward_parent
						} else {
							unreachable!()
						};
					}

					for kind in kinds.into_iter().rev() {
						self.builder.start_node(P::Language::kind_to_raw(kind));
					}
				},
				Event::AddToken => self.token(),
				Event::FinishNode => self.builder.finish_node(),
				Event::Error(error) => self.errors.push(error),
				Event::Placeholder => {},
			}

			self.eat_trivia();
		}

		Parse {
			green_node: self.builder.finish(),
			errors: self.errors,
			_marker: PhantomData,
		}
	}

	fn eat_trivia(&mut self) {
		while let Some(token) = self.tokens.get(self.cursor) {
			if !token.kind.is_trivia() {
				break;
			}

			self.token();
		}
	}

	fn token(&mut self) {
		let Token { kind, text, .. } = self.tokens[self.cursor];

		self.builder
			.token(P::Language::kind_to_raw(kind.into()), text);

		self.cursor += 1;
	}
}
