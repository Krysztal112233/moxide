pub(crate) trait Property {
    fn merge(self, income: Self) -> Self;
}
