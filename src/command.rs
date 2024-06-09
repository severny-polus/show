pub enum Command<M> {
    Phantom(M),
    Update,
    Terminate,
    None,
}
