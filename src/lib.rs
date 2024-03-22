#[derive(Debug)]
pub struct ChessPosition{
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen{
    rank: i32,
    file: i32,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 1 || rank > 8 || file < 1 || file > 8 {
            None
        } else {
            Some(ChessPosition{rank, file})
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        // unimplemented!("Given the chess position {position:?}, construct a Queen struct.");
        Queen {
                rank: position.rank,
                file: position.file,

            }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        //unimplemented!("Determine if this Queen can attack the other Queen {other:?}");
        self.rank == other.rank || self.file == other.file || (self.rank - other.rank).abs() == (self.file - other.file).abs()
    }
}
