// Signatures
one sig Text {}
one sig EncryptedText {}

sig User {
  userId: Int,
  userName: Text,
  password: EncryptedText,
  criteria: Int,
  criteriaCurrency: Text,
  email: Text
}

sig CreditCard {
  number: Int,
  expiryDate: Int,
  cvv: Int,
  user: one User,
  limit: Int,
  currentBalance: Int
}

sig Item {
  name: Text,
  price: Int,
  quantityInStock: Int
}

sig Order {
  items: some Item,
  totalPrice: Int,
  paymentMethod: one PaymentMethod
}

sig PaymentMethod {
  kind: one CreditCard + CashPayment
}

sig CashPayment extends PaymentMethod {}
sig CreditPayment extends PaymentMethod {
  card: one CreditCard
}

// Facts
fact UserHasCreditCard {
  all u: User | one c: CreditCard | c.user = u
}

fact OrderHasPaymentMethod {
  all o: Order | one p: PaymentMethod | o.paymentMethod = p
}

fact CreditPaymentHasCard {
  all p: CreditPayment | p.card in CreditCard
}

fact CreditCardPaymentLimit {
  all p: CreditPayment | all o: Order | p.card.currentBalance + o.totalPrice <= p.card.limit
}

fact OrderItemQuantityConstraint {
  all o: Order, i: o.items | i.quantityInStock > 1 
}

// Assertions
assert NoCreditPurchasesWhenMaxedOut {
  all p: CreditPayment | all o: Order | p.card.currentBalance + o.totalPrice <= p.card.limit
}
pred buying {
  all p: CreditPayment | all o: Order | p.card.currentBalance + o.totalPrice <= p.card.limit
}

assert ItemsInStock {
  all o: Order | all i: o.items | i.quantityInStock >= 1
}

// Commands
run buying for 4 Order, 1 User, 1 CreditCard, 2 CashPayment, 2 CreditPayment, 5 Item, 3 PaymentMethod
check NoCreditPurchasesWhenMaxedOut for 3 Order, 2 User, 2 CreditCard, 2 CashPayment, 2 CreditPayment, 5 Item, 3 PaymentMethod
check ItemsInStock for 4 Order, 1 User, 1 CreditCard, 2 CashPayment, 3 CreditPayment, 5 Item, 3 PaymentMethod

