// Definisi entitas
abstract sig User {}
sig Customer extends User {}
sig Merchant extends User {}

sig CreditCard {
  owner: Customer,
  limit: Int,
  balance: Int
}

sig Transaction {
  payer: Customer,
  payee: Merchant,
  amount: Int,
  paymentType: one PaymentType,
  status: one TransactionStatus
}

sig TransactionStatus {
  success, failure
}

sig PaymentType {
  credit: one Transaction,
  debit: one Transaction
}

sig BankAPI {
  validateTransaction: Transaction -> TransactionStatus,
  processPayment: Transaction -> TransactionStatus
}

// Aturan atau properti sistem
fact SufficientBalance {
  all t: Transaction |
    t.payer.creditCard.balance >= t.amount
}

fact CreditLimit {
  all c: CreditCard |
    c.balance <= c.limit
}

// Contoh instance
run {
  // Definisi beberapa pengguna, kartu kredit, transaksi, dan BankAPI
  some c1, c2: Customer |
  some m1: Merchant |
  some cc1: CreditCard | cc1.owner = c1 and cc1.limit = 1000 and cc1.balance = 500 |
  some cc2: CreditCard | cc2.owner = c2 and cc2.limit = 2000 and cc2.balance = 1500 |
  some t_success: Transaction | t_success.payer = c1 and t_success.payee = m1 and t_success.amount = 200 and t_success.paymentType = (credit + debit) and t_success.status = success |
  some t_failure: Transaction | t_failure.payer = c2 and t_failure.payee = m1 and t_failure.amount = 1000 and t_failure.paymentType = (credit + debit) and t_failure.status = failure |
  some pt1, pt2: PaymentType | t_success.paymentType = pt1 and t_failure.paymentType = pt2 |
  some s1, s2: TransactionStatus | t_success.status = s1 and t_failure.status = s2 |
  some bankAPI: BankAPI | all t: Transaction | bankAPI.validateTransaction[t] = s1 and bankAPI.processPayment[t] = s2
}

// Diagram Kelas Sederhana
abstract sig User { }
sig Customer, Merchant extends User { }
sig CreditCard {
  owner: Customer,
  limit: Int,
  balance: Int
}
sig Transaction {
  payer: Customer,
  payee: Merchant,
  amount: Int,
  paymentType: one PaymentType,
  status: one TransactionStatus
}
sig TransactionStatus {
  success, failure
}
sig PaymentType {
  credit: one Transaction,
  debit: one Transaction
}
sig BankAPI {
  validateTransaction: Transaction -> TransactionStatus,
  processPayment: Transaction -> TransactionStatus
}

// Sequence untuk Skenario
pred PaymentScenario {
  some c1, c2: Customer |
  some m1: Merchant |
  some cc1: CreditCard | cc1.owner = c1 and cc1.limit = 1000 and cc1.balance = 500 |
  some cc2: CreditCard | cc2.owner = c2 and cc2.limit = 2000 and cc2.balance = 1500 |
  some t_success: Transaction | t_success.payer = c1 and t_success.payee = m1 and t_success.amount = 200 and t_success.paymentType = (credit + debit) and t_success.status = success |
  some t_failure: Transaction | t_failure.payer = c2 and t_failure.payee = m1 and t_failure.amount = 1000 and t_failure.paymentType = (credit + debit) and t_failure.status = failure |
  some pt1, pt2: PaymentType | t_success.paymentType = pt1 and t_failure.paymentType = pt2 |
  some s1, s2: TransactionStatus | t_success.status = s1 and t_failure.status = s2 |
  some bankAPI: BankAPI | all t: Transaction | bankAPI.validateTransaction[t] = s1 and bankAPI.processPayment[t] = s2
}

run PaymentScenario for 5
