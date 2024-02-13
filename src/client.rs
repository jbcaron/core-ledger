enum Query {
	GetAccount(Vec<u8>),
	AddTransaction(Transaction),
}