use clinvoice_finance::{Currency, ExchangeRates, Exchangeable};

use super::MatchJob;

impl Exchangeable for MatchJob
{
	fn exchange(self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			invoice: self.invoice.exchange(currency, rates),
			..self
		}
	}

	fn exchange_ref(&self, currency: Currency, rates: &ExchangeRates) -> Self
	{
		Self {
			client: self.client.clone(),
			date_close: self.date_close.clone(),
			date_open: self.date_open.clone(),
			id: self.id.clone(),
			increment: self.increment.clone(),
			invoice: self.invoice.exchange_ref(currency, rates),
			notes: self.notes.clone(),
			objectives: self.objectives.clone(),
		}
	}
}
