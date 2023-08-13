
  #[test] fn demo_debug_format() {
    #[derive(Copy,Clone,Debug,PartialEq,Eq)]pub enum PieceShape {King,Queen,Rook,Bishop,Knight,Pawn}
    let q = PieceShape::Queen;
    let p = PieceShape::Pawn;
    let k = PieceShape::King;
    // println!("q={:?} p={:?} k={:?}", q, p, k);
  }