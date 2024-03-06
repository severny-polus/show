pub enum Command<M> {
    Phantom(M),
    Update,
    None,
}
