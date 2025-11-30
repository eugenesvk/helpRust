/// An `Iterator`-like trait that can borrow from `Self`
trait LendingIterator {
  type Item<'a> where Self: 'a;
  fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

/// Can be implemented over smart pointers, like `Rc` or `Arc`, in order to allow being generic over the pointer type
trait PointerFamily {
  type Pointer<T>: Deref<Target = T>;
  fn new<T>(value: T) -> Self::Pointer<T>;
}

/// Allows borrowing an array of items. Useful for `NdArray`-like types that don't necessarily store data contiguously.
trait BorrowArray<T> {
  type Array<'x, const N: usize> where Self: 'x;
  fn borrow_array<'a, const N: usize>(&'a self) -> Self::Array<'a, N>;
}
