pub enum Choice<A, B> {
    A(A),
    B(B),
}

impl<A, B> Choice<A, B> {
    pub fn is_a(&self) -> bool {
        match self {
            Choice::A(_) => true,
            _ => false
        }
    }

    pub fn is_b(&self) -> bool {
        match self {
            Choice::B(_) => true,
            _ => false
        }
    }

    pub fn unwrap_a(self) -> A {
        match self {
            Choice::A(a) => a,
            _ => panic!("Tried to unwrap choice A, but choice B was present!")
        }
    }

    pub fn unwrap_b(self) -> B {
        match self {
            Choice::B(b) => b,
            _ => panic!("Tried to unwrap choice B, but choice A was present!")
        }
    }

    pub fn map<FA, FB, T, U>(self, map_a: FA, map_b: FB) -> Choice<T, U>
        where FA: FnOnce(A) -> T, FB: FnOnce(B) -> U
    {
        match self {
            Choice::A(a) => Choice::A(map_a(a)),
            Choice::B(b) => Choice::B(map_b(b)),
        }
    }

    pub fn map_a<F, T>(self, map: F) -> Choice<T, B>
        where F: FnOnce(A) -> T
    {
        match self {
            Choice::A(a) => Choice::A(map(a)),
            Choice::B(b) => Choice::B(b),
        }
    }

    pub fn map_b<F, T>(self, map: F) -> Choice<A, T>
        where F: FnOnce(B) -> T
    {
        match self {
            Choice::A(a) => Choice::A(a),
            Choice::B(b) => Choice::B(map(b)),
        }
    }
}

impl<T> Choice<T, T> {
    pub fn unwrap(self) -> T {
        match self {
            Choice::A(val) | Choice::B(val) => val,
        }
    }
}
