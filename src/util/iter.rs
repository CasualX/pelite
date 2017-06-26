
macro_rules! def_iter {
	(struct $name:ident -> $raw:ty, $item:ty; $this:ident $map:expr) => {
		impl<'a, P: Pe<'a> + Copy> $name<'a, P> {
			pub fn as_slice(&self) -> &'a [$raw] {
				self.iter.as_slice()
			}
		}
		impl<'a, P: Pe<'a> + Copy> Iterator for $name<'a, P> {
			type Item = $item;
			#[inline]
			fn next(&mut self) -> Option<$item> {
				let $this = self;
				$this.iter.next().map($map)
			}
			#[inline]
			fn size_hint(&self) -> (usize, Option<usize>) {
				self.iter.size_hint()
			}
			#[inline]
			fn count(self) -> usize {
				self.iter.as_slice().len()
			}
			#[inline]
			fn nth(&mut self, n: usize) -> Option<$item> {
				let $this = self;
				$this.iter.nth(n).map($map)
			}
			// fn last(self) -> Option<$item> {
			// 	let mut $this = self;
			// 	$this.iter.last().map($map)
			// }
		}
		impl<'a, P: Pe<'a> + Copy> ExactSizeIterator for $name<'a, P> {}
		impl<'a, P: Pe<'a> + Copy> DoubleEndedIterator for $name<'a, P> {
			#[inline]
			fn next_back(&mut self) -> Option<$item> {
				let $this = self;
				$this.iter.next_back().map($map)
			}
		}
	};
}
